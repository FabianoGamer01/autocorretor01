use ort::session::Session;
use ort::value::Tensor;
use std::path::Path;
use tokenizers::Tokenizer;

pub struct StageC {
    session: Option<Session>,
    tokenizer: Option<Tokenizer>,
}

impl StageC {
    pub fn new() -> Self {
        Self {
            session: None,
            tokenizer: None,
        }
    }

    /// Inicializa o motor se os arquivos existirem no caminho especificado.
    /// Garante execução 100% local ao buscar apenas em arquivos locais.
    pub fn init_from_dir<P: AsRef<Path>>(&mut self, base_dir: P) {
        let base_dir = base_dir.as_ref();
        let model_path = base_dir.join("model.onnx");
        let tokenizer_path = base_dir.join("tokenizer.json");

        if model_path.exists() && tokenizer_path.exists() {
            match self.load_model(&model_path, &tokenizer_path) {
                Ok(_) => {
                    // Modelo carregado com sucesso
                }
                Err(e) => {
                    // Falha silenciosa: Stage C é opcional
                    eprintln!("[StageC] Falha ao carregar modelo: {}", e);
                }
            }
        }
    }

    pub fn is_ready(&self) -> bool {
        self.session.is_some() && self.tokenizer.is_some()
    }

    pub fn load_model<P: AsRef<Path>>(
        &mut self,
        model_path: P,
        tokenizer_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session = Session::builder()?
            .with_intra_threads(1)?
            .commit_from_file(model_path)?;

        let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(|e| e.to_string())?;

        self.session = Some(session);
        self.tokenizer = Some(tokenizer);

        Ok(())
    }

    /// Executa inferência ONNX para corrigir o texto.
    /// Retorna `Some(corrected)` se o modelo produziu uma correção diferente do input,
    /// ou `None` se o modelo não está disponível ou não alterou o texto.
    pub fn predict(&mut self, text: &str) -> Option<String> {
        let tokenizer = self.tokenizer.as_ref()?;
        let session = self.session.as_mut()?;

        // 1. Tokenizar o texto de entrada
        let encoding = tokenizer.encode(text, true).ok()?;
        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let attention_mask: Vec<i64> = encoding
            .get_attention_mask()
            .iter()
            .map(|&m| m as i64)
            .collect();

        if input_ids.is_empty() {
            return None;
        }

        let seq_len = input_ids.len();

        // 2. Criar tensores de entrada [1, seq_len]
        let input_ids_tensor =
            Tensor::<i64>::from_array(([1usize, seq_len], input_ids.clone())).ok()?;
        let attention_mask_tensor =
            Tensor::<i64>::from_array(([1usize, seq_len], attention_mask)).ok()?;

        // 3. Executar a sessão ONNX
        let outputs = session
            .run(ort::inputs![
                "input_ids" => input_ids_tensor,
                "attention_mask" => attention_mask_tensor,
            ])
            .ok()?;

        // 4. Extrair logits do primeiro output (index 0)
        // API do ort v2: try_extract_tensor retorna Result<(&Shape, &[T])>
        // Shape implementa Deref<Target=[i64]>, então shape[0] = batch, shape[1] = seq_len, shape[2] = vocab_size
        let (logits_shape, logits_data) = outputs[0].try_extract_tensor::<f32>().ok()?;

        if logits_shape.len() < 3 {
            return None;
        }

        let vocab_size = logits_shape[2] as usize;

        // 5. Argmax por posição para obter os token IDs corrigidos
        let mut predicted_ids: Vec<u32> = Vec::with_capacity(seq_len);
        for pos in 0..seq_len {
            let start = pos * vocab_size;
            let end = start + vocab_size;
            if end > logits_data.len() {
                break;
            }
            let slice = &logits_data[start..end];
            let best_id = slice
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx as u32)
                .unwrap_or(input_ids[pos] as u32);
            predicted_ids.push(best_id);
        }

        // 6. Decodificar os IDs preditos de volta para texto
        let corrected = tokenizer.decode(&predicted_ids, true).ok()?;

        // 7. Só retorna se a correção for diferente do input
        if corrected.trim() != text.trim() && !corrected.trim().is_empty() {
            Some(corrected.trim().to_string())
        } else {
            None
        }
    }
}
