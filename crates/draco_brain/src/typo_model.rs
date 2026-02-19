use std::collections::HashMap;

/// Modelo de correção de erros de digitação comuns no PT-BR.
/// Mapeia palavras completas digitadas sem acentos para a forma correta,
/// e sufixos comuns para correção de terminações.
///
/// REGRAS:
/// - Nunca inserir entradas "word → word" (inúteis e bloqueiam pipeline)
/// - Palavras ambíguas (nos/nós, esta/está) ficam FORA — tratadas por frequência
/// - Sem duplicatas
pub struct TypoModel {
    word_map: HashMap<String, String>,
    suffix_map: Vec<(String, String)>,
}

impl TypoModel {
    pub fn new() -> Self {
        let mut m = HashMap::with_capacity(512);

        // ═══════════════════════════════════════════════════
        // SEÇÃO 1: Abreviações de internet / chat
        // ═══════════════════════════════════════════════════
        m.insert("vc".into(), "você".into());
        m.insert("vcs".into(), "vocês".into());
        m.insert("tb".into(), "também".into());
        m.insert("tbm".into(), "também".into());
        m.insert("tmb".into(), "também".into());
        m.insert("td".into(), "tudo".into());
        m.insert("mt".into(), "muito".into());
        m.insert("mto".into(), "muito".into());
        m.insert("mta".into(), "muita".into());
        m.insert("mts".into(), "muitos".into());
        m.insert("mtas".into(), "muitas".into());
        m.insert("msm".into(), "mesmo".into());
        m.insert("msg".into(), "mensagem".into());
        m.insert("msgs".into(), "mensagens".into());
        m.insert("pq".into(), "porque".into());
        m.insert("qdo".into(), "quando".into());
        m.insert("qnd".into(), "quando".into());
        m.insert("qto".into(), "quanto".into());
        m.insert("qta".into(), "quanta".into());
        m.insert("qtos".into(), "quantos".into());
        m.insert("qtas".into(), "quantas".into());
        m.insert("qq".into(), "qualquer".into());
        m.insert("cmg".into(), "comigo".into());
        m.insert("ctg".into(), "contigo".into());
        m.insert("hj".into(), "hoje".into());
        m.insert("dps".into(), "depois".into());
        m.insert("obg".into(), "obrigado".into());
        m.insert("obgd".into(), "obrigado".into());
        m.insert("blz".into(), "beleza".into());
        m.insert("flw".into(), "falou".into());
        m.insert("vlw".into(), "valeu".into());
        m.insert("agr".into(), "agora".into());
        m.insert("nd".into(), "nada".into());
        m.insert("nda".into(), "nada".into());
        m.insert("ngm".into(), "ninguém".into());
        m.insert("pfv".into(), "por favor".into());
        m.insert("pfvr".into(), "por favor".into());
        m.insert("pf".into(), "por favor".into());
        m.insert("tdo".into(), "tudo".into());
        m.insert("tda".into(), "toda".into());
        m.insert("tds".into(), "todos".into());
        m.insert("amg".into(), "amigo".into());
        m.insert("amgs".into(), "amigos".into());

        // ═══════════════════════════════════════════════════
        // SEÇÃO 2: Pares de confusão (palavra válida → mais provável)
        //          Verificados ANTES do dicionário no stage_a
        // ═══════════════════════════════════════════════════
        m.insert("par".into(), "para".into());
        m.insert("eh".into(), "é".into());
        m.insert("la".into(), "lá".into());
        m.insert("ca".into(), "cá".into());
        m.insert("ja".into(), "já".into());
        m.insert("so".into(), "só".into());
        m.insert("ai".into(), "aí".into());
        m.insert("pe".into(), "pé".into());
        m.insert("fe".into(), "fé".into());
        m.insert("pro".into(), "pró".into());

        // ═══════════════════════════════════════════════════
        // SEÇÃO 3: Expressões compostas comuns escritas juntas
        // ═══════════════════════════════════════════════════
        m.insert("oque".into(), "o que".into());
        m.insert("oq".into(), "o que".into());
        m.insert("dnv".into(), "de novo".into());
        m.insert("porisso".into(), "por isso".into());
        m.insert("apartir".into(), "a partir".into());
        m.insert("derrepente".into(), "de repente".into());
        m.insert("concerteza".into(), "com certeza".into());
        m.insert("agente".into(), "a gente".into());
        m.insert("afim".into(), "a fim".into());
        m.insert("oque".into(), "o que".into());
        m.insert("aonde".into(), "aonde".into()); // válida, manter

        // ═══════════════════════════════════════════════════
        // SEÇÃO 4: Palavras sem acento → forma correta
        // ═══════════════════════════════════════════════════

        // --- ão / ã ---
        m.insert("nao".into(), "não".into());
        m.insert("sao".into(), "são".into());
        m.insert("pao".into(), "pão".into());
        m.insert("mao".into(), "mão".into());
        m.insert("cao".into(), "cão".into());
        m.insert("vao".into(), "vão".into());
        m.insert("dao".into(), "dão".into());
        m.insert("sera".into(), "será".into());
        m.insert("estao".into(), "estão".into());
        m.insert("entao".into(), "então".into());
        m.insert("tambem".into(), "também".into());
        m.insert("voce".into(), "você".into());
        m.insert("voces".into(), "vocês".into());
        m.insert("porem".into(), "porém".into());
        m.insert("alem".into(), "além".into());
        m.insert("ate".into(), "até".into());
        m.insert("irma".into(), "irmã".into());
        m.insert("manha".into(), "manhã".into());
        m.insert("amanha".into(), "amanhã".into());
        m.insert("irmao".into(), "irmão".into());
        m.insert("cidadao".into(), "cidadão".into());
        m.insert("alemao".into(), "alemão".into());
        m.insert("capitao".into(), "capitão".into());
        m.insert("campeao".into(), "campeão".into());
        m.insert("leao".into(), "leão".into());
        m.insert("limao".into(), "limão".into());
        m.insert("botao".into(), "botão".into());
        m.insert("cartao".into(), "cartão".into());
        m.insert("aviao".into(), "avião".into());
        m.insert("verao".into(), "verão".into());
        m.insert("orgao".into(), "órgão".into());
        m.insert("orgaos".into(), "órgãos".into());
        m.insert("orfao".into(), "órfão".into());
        m.insert("orfaos".into(), "órfãos".into());
        m.insert("sertao".into(), "sertão".into());
        m.insert("maos".into(), "mãos".into());
        m.insert("irmaos".into(), "irmãos".into());
        m.insert("cidadaos".into(), "cidadãos".into());
        m.insert("alemoes".into(), "alemães".into());
        m.insert("capitaes".into(), "capitães".into());
        m.insert("campeoes".into(), "campeões".into());
        m.insert("leoes".into(), "leões".into());
        m.insert("limoes".into(), "limões".into());
        m.insert("botoes".into(), "botões".into());
        m.insert("cartoes".into(), "cartões".into());
        m.insert("avioes".into(), "aviões".into());
        m.insert("veroes".into(), "verões".into());
        m.insert("sertoes".into(), "sertões".into());
        m.insert("coracoes".into(), "corações".into());

        // --- ç ---
        m.insert("comeco".into(), "começo".into());
        m.insert("cabeca".into(), "cabeça".into());
        m.insert("braco".into(), "braço".into());
        m.insert("coracao".into(), "coração".into());
        m.insert("situacao".into(), "situação".into());
        m.insert("solucao".into(), "solução".into());
        m.insert("atencao".into(), "atenção".into());
        m.insert("informacao".into(), "informação".into());
        m.insert("comunicacao".into(), "comunicação".into());
        m.insert("educacao".into(), "educação".into());
        m.insert("producao".into(), "produção".into());
        m.insert("funcao".into(), "função".into());
        m.insert("relacao".into(), "relação".into());
        m.insert("posicao".into(), "posição".into());
        m.insert("condicao".into(), "condição".into());
        m.insert("operacao".into(), "operação".into());
        m.insert("aplicacao".into(), "aplicação".into());
        m.insert("configuracao".into(), "configuração".into());
        m.insert("instalacao".into(), "instalação".into());
        m.insert("atualizacao".into(), "atualização".into());
        m.insert("versao".into(), "versão".into());
        m.insert("conexao".into(), "conexão".into());
        m.insert("excecao".into(), "exceção".into());
        m.insert("secao".into(), "seção".into());
        m.insert("colecao".into(), "coleção".into());
        m.insert("selecao".into(), "seleção".into());
        m.insert("direcao".into(), "direção".into());
        m.insert("protecao".into(), "proteção".into());
        m.insert("percepcao".into(), "percepção".into());
        m.insert("descricao".into(), "descrição".into());
        m.insert("inscricao".into(), "inscrição".into());
        m.insert("prescricao".into(), "prescrição".into());
        m.insert("restricao".into(), "restrição".into());
        m.insert("acao".into(), "ação".into());
        m.insert("nacoes".into(), "nações".into());
        m.insert("acoes".into(), "ações".into());
        m.insert("licao".into(), "lição".into());
        m.insert("nocao".into(), "noção".into());
        m.insert("porcao".into(), "porção".into());
        m.insert("forca".into(), "força".into());
        m.insert("forcas".into(), "forças".into());
        m.insert("preco".into(), "preço".into());
        m.insert("traco".into(), "traço".into());
        m.insert("espaco".into(), "espaço".into());
        m.insert("cancao".into(), "canção".into());
        m.insert("nacao".into(), "nação".into());
        m.insert("estacao".into(), "estação".into());
        m.insert("sensacao".into(), "sensação".into());
        m.insert("tentacao".into(), "tentação".into());
        m.insert("criacao".into(), "criação".into());
        m.insert("variacao".into(), "variação".into());
        m.insert("avaliacao".into(), "avaliação".into());
        m.insert("negociacao".into(), "negociação".into());
        m.insert("organizacao".into(), "organização".into());
        m.insert("realizacao".into(), "realização".into());
        m.insert("utilizacao".into(), "utilização".into());
        m.insert("participacao".into(), "participação".into());
        m.insert("apresentacao".into(), "apresentação".into());
        m.insert("representacao".into(), "representação".into());
        m.insert("implementacao".into(), "implementação".into());
        m.insert("documentacao".into(), "documentação".into());
        m.insert("autenticacao".into(), "autenticação".into());
        m.insert("autorizacao".into(), "autorização".into());
        m.insert("integracao".into(), "integração".into());
        m.insert("geracao".into(), "geração".into());
        m.insert("publicacao".into(), "publicação".into());
        m.insert("notificacao".into(), "notificação".into());
        m.insert("localizacao".into(), "localização".into());
        m.insert("sincronizacao".into(), "sincronização".into());
        m.insert("personalizacao".into(), "personalização".into());
        m.insert("otimizacao".into(), "otimização".into());
        m.insert("visualizacao".into(), "visualização".into());
        m.insert("inicializacao".into(), "inicialização".into());
        m.insert("finalizacao".into(), "finalização".into());
        m.insert("verificacao".into(), "verificação".into());
        m.insert("validacao".into(), "validação".into());
        m.insert("importacao".into(), "importação".into());
        m.insert("exportacao".into(), "exportação".into());
        m.insert("traducao".into(), "tradução".into());
        m.insert("reducao".into(), "redução".into());
        m.insert("deducao".into(), "dedução".into());
        m.insert("adicao".into(), "adição".into());
        m.insert("subtracao".into(), "subtração".into());
        m.insert("multiplicacao".into(), "multiplicação".into());
        m.insert("divisao".into(), "divisão".into());
        m.insert("precisao".into(), "precisão".into());
        m.insert("decisao".into(), "decisão".into());
        m.insert("revisao".into(), "revisão".into());
        m.insert("previsao".into(), "previsão".into());
        m.insert("invasao".into(), "invasão".into());
        m.insert("evasao".into(), "evasão".into());
        m.insert("expansao".into(), "expansão".into());
        m.insert("extensao".into(), "extensão".into());
        m.insert("tensao".into(), "tensão".into());
        m.insert("pensao".into(), "pensão".into());
        m.insert("mansao".into(), "mansão".into());
        m.insert("missao".into(), "missão".into());
        m.insert("emissao".into(), "emissão".into());
        m.insert("transmissao".into(), "transmissão".into());
        m.insert("permissao".into(), "permissão".into());
        m.insert("comissao".into(), "comissão".into());
        m.insert("omissao".into(), "omissão".into());
        m.insert("admissao".into(), "admissão".into());
        m.insert("discussao".into(), "discussão".into());
        m.insert("sessao".into(), "sessão".into());
        m.insert("expressao".into(), "expressão".into());
        m.insert("impressao".into(), "impressão".into());
        m.insert("depressao".into(), "depressão".into());
        m.insert("compressao".into(), "compressão".into());
        m.insert("profissao".into(), "profissão".into());
        m.insert("confissao".into(), "confissão".into());
        m.insert("agressao".into(), "agressão".into());
        m.insert("regressao".into(), "regressão".into());
        m.insert("progressao".into(), "progressão".into());
        m.insert("obsessao".into(), "obsessão".into());
        m.insert("possessao".into(), "possessão".into());
        m.insert("processao".into(), "procissão".into());
        m.insert("excursao".into(), "excursão".into());
        m.insert("excursoes".into(), "excursões".into());
        m.insert("conclusao".into(), "conclusão".into());
        m.insert("exclusao".into(), "exclusão".into());
        m.insert("inclusao".into(), "inclusão".into());
        m.insert("ilusao".into(), "ilusão".into());
        m.insert("fusao".into(), "fusão".into());
        m.insert("confusao".into(), "confusão".into());
        m.insert("difusao".into(), "difusão".into());
        m.insert("infusao".into(), "infusão".into());
        m.insert("transfusao".into(), "transfusão".into());
        m.insert("reclusao".into(), "reclusão".into());
        m.insert("oclusao".into(), "oclusão".into());

        // --- é / ê ---
        m.insert("cafe".into(), "café".into());
        m.insert("tres".into(), "três".into());
        m.insert("ingles".into(), "inglês".into());
        m.insert("frances".into(), "francês".into());
        m.insert("holandes".into(), "holandês".into());
        m.insert("japones".into(), "japonês".into());
        m.insert("chines".into(), "chinês".into());
        m.insert("portugues".into(), "português".into());
        m.insert("mes".into(), "mês".into());
        m.insert("pes".into(), "pés".into());
        m.insert("cafes".into(), "cafés".into());

        // --- ó / ô ---
        m.insert("pos".into(), "pós".into());
        m.insert("avos".into(), "avós".into());

        // --- í / ú / proparoxítonas ---
        m.insert("atras".into(), "atrás".into());
        m.insert("apos".into(), "após".into());
        m.insert("proprio".into(), "próprio".into());
        m.insert("propria".into(), "própria".into());
        m.insert("proprios".into(), "próprios".into());
        m.insert("publico".into(), "público".into());
        m.insert("publica".into(), "pública".into());
        m.insert("publicos".into(), "públicos".into());
        m.insert("publicas".into(), "públicas".into());
        m.insert("unico".into(), "único".into());
        m.insert("unica".into(), "única".into());
        m.insert("unicos".into(), "únicos".into());
        m.insert("unicas".into(), "únicas".into());
        m.insert("util".into(), "útil".into());
        m.insert("uteis".into(), "úteis".into());
        m.insert("facil".into(), "fácil".into());
        m.insert("faceis".into(), "fáceis".into());
        m.insert("dificil".into(), "difícil".into());
        m.insert("dificeis".into(), "difíceis".into());
        m.insert("agil".into(), "ágil".into());
        m.insert("ageis".into(), "ágeis".into());
        m.insert("fragil".into(), "frágil".into());
        m.insert("frageis".into(), "frágeis".into());
        m.insert("fertil".into(), "fértil".into());
        m.insert("ferteis".into(), "férteis".into());
        m.insert("habil".into(), "hábil".into());
        m.insert("habeis".into(), "hábeis".into());
        m.insert("imovel".into(), "imóvel".into());
        m.insert("imoveis".into(), "imóveis".into());
        m.insert("movel".into(), "móvel".into());
        m.insert("moveis".into(), "móveis".into());
        m.insert("nivel".into(), "nível".into());
        m.insert("niveis".into(), "níveis".into());

        // --- Adjetivos com acento (proparoxítonas) ---
        m.insert("otimo".into(), "ótimo".into());
        m.insert("otima".into(), "ótima".into());
        m.insert("otimos".into(), "ótimos".into());
        m.insert("otimas".into(), "ótimas".into());
        m.insert("pessimo".into(), "péssimo".into());
        m.insert("pessima".into(), "péssima".into());
        m.insert("pessimos".into(), "péssimos".into());
        m.insert("pessimas".into(), "péssimas".into());
        m.insert("maximo".into(), "máximo".into());
        m.insert("maxima".into(), "máxima".into());
        m.insert("minimo".into(), "mínimo".into());
        m.insert("minima".into(), "mínima".into());
        m.insert("proximo".into(), "próximo".into());
        m.insert("proxima".into(), "próxima".into());
        m.insert("proximos".into(), "próximos".into());
        m.insert("proximas".into(), "próximas".into());
        m.insert("ultimo".into(), "último".into());
        m.insert("ultima".into(), "última".into());
        m.insert("ultimos".into(), "últimos".into());
        m.insert("ultimas".into(), "últimas".into());
        m.insert("numero".into(), "número".into());
        m.insert("numeros".into(), "números".into());
        m.insert("pagina".into(), "página".into());
        m.insert("paginas".into(), "páginas".into());
        m.insert("codigo".into(), "código".into());
        m.insert("codigos".into(), "códigos".into());
        m.insert("periodo".into(), "período".into());
        m.insert("periodos".into(), "períodos".into());
        m.insert("metodo".into(), "método".into());
        m.insert("metodos".into(), "métodos".into());
        m.insert("modulo".into(), "módulo".into());
        m.insert("modulos".into(), "módulos".into());
        m.insert("indice".into(), "índice".into());
        m.insert("indices".into(), "índices".into());
        m.insert("titulo".into(), "título".into());
        m.insert("titulos".into(), "títulos".into());
        m.insert("capitulo".into(), "capítulo".into());
        m.insert("capitulos".into(), "capítulos".into());
        m.insert("topico".into(), "tópico".into());
        m.insert("topicos".into(), "tópicos".into());
        m.insert("logico".into(), "lógico".into());
        m.insert("logica".into(), "lógica".into());
        m.insert("logicos".into(), "lógicos".into());
        m.insert("logicas".into(), "lógicas".into());
        m.insert("basico".into(), "básico".into());
        m.insert("basica".into(), "básica".into());
        m.insert("basicos".into(), "básicos".into());
        m.insert("basicas".into(), "básicas".into());
        m.insert("tecnico".into(), "técnico".into());
        m.insert("tecnica".into(), "técnica".into());
        m.insert("tecnicos".into(), "técnicos".into());
        m.insert("tecnicas".into(), "técnicas".into());
        m.insert("fisico".into(), "físico".into());
        m.insert("fisica".into(), "física".into());
        m.insert("fisicos".into(), "físicos".into());
        m.insert("fisicas".into(), "físicas".into());
        m.insert("quimico".into(), "químico".into());
        m.insert("quimica".into(), "química".into());
        m.insert("quimicos".into(), "químicos".into());
        m.insert("quimicas".into(), "químicas".into());
        m.insert("matematico".into(), "matemático".into());
        m.insert("matematica".into(), "matemática".into());
        m.insert("historico".into(), "histórico".into());
        m.insert("historica".into(), "histórica".into());
        m.insert("historicos".into(), "históricos".into());
        m.insert("historicas".into(), "históricas".into());
        m.insert("economico".into(), "econômico".into());
        m.insert("economica".into(), "econômica".into());
        m.insert("economicos".into(), "econômicos".into());
        m.insert("economicas".into(), "econômicas".into());
        m.insert("politico".into(), "político".into());
        m.insert("politica".into(), "política".into());
        m.insert("politicos".into(), "políticos".into());
        m.insert("politicas".into(), "políticas".into());
        m.insert("juridico".into(), "jurídico".into());
        m.insert("juridica".into(), "jurídica".into());
        m.insert("juridicos".into(), "jurídicos".into());
        m.insert("juridicas".into(), "jurídicas".into());
        m.insert("medico".into(), "médico".into());
        m.insert("medica".into(), "médica".into());
        m.insert("medicos".into(), "médicos".into());
        m.insert("medicas".into(), "médicas".into());
        m.insert("academico".into(), "acadêmico".into());
        m.insert("academica".into(), "acadêmica".into());
        m.insert("academicos".into(), "acadêmicos".into());
        m.insert("academicas".into(), "acadêmicas".into());
        m.insert("eletronico".into(), "eletrônico".into());
        m.insert("eletronica".into(), "eletrônica".into());
        m.insert("eletronicos".into(), "eletrônicos".into());
        m.insert("eletronicas".into(), "eletrônicas".into());
        m.insert("automatico".into(), "automático".into());
        m.insert("automatica".into(), "automática".into());
        m.insert("automaticos".into(), "automáticos".into());
        m.insert("automaticas".into(), "automáticas".into());
        m.insert("democratico".into(), "democrático".into());
        m.insert("democratica".into(), "democrática".into());
        m.insert("democraticos".into(), "democráticos".into());
        m.insert("democraticas".into(), "democráticas".into());
        m.insert("fantastico".into(), "fantástico".into());
        m.insert("fantastica".into(), "fantástica".into());
        m.insert("fantasticos".into(), "fantásticos".into());
        m.insert("fantasticas".into(), "fantásticas".into());
        m.insert("plastico".into(), "plástico".into());
        m.insert("plastica".into(), "plástica".into());
        m.insert("plasticos".into(), "plásticos".into());
        m.insert("plasticas".into(), "plásticas".into());
        m.insert("elastico".into(), "elástico".into());
        m.insert("elastica".into(), "elástica".into());
        m.insert("elasticos".into(), "elásticos".into());
        m.insert("elasticas".into(), "elásticas".into());
        m.insert("drastico".into(), "drástico".into());
        m.insert("drastica".into(), "drástica".into());
        m.insert("drasticos".into(), "drásticos".into());
        m.insert("drasticas".into(), "drásticas".into());

        // --- Erros de digitação por teclas próximas ---
        m.insert("qeu".into(), "que".into());
        m.insert("nad".into(), "nada".into());

        // ═══════════════════════════════════════════════════
        // SEÇÃO 5: Sufixos (aplicados a palavras >4 chars)
        // ═══════════════════════════════════════════════════
        let suffix_map = vec![
            ("cao".into(), "ção".into()),
            ("oes".into(), "ões".into()),
            ("ao".into(), "ão".into()),
        ];

        Self {
            word_map: m,
            suffix_map,
        }
    }

    /// Tenta corrigir uma palavra completa (já em minúsculas).
    pub fn get_correction(&self, word_lower: &str) -> Option<String> {
        // 1. Verificação exata
        if let Some(corrected) = self.word_map.get(word_lower) {
            return Some(corrected.clone());
        }

        // 2. Verificação por sufixo (palavras >4 letras)
        if word_lower.len() > 4 {
            for (suffix, replacement) in &self.suffix_map {
                if word_lower.ends_with(suffix.as_str()) {
                    let prefix = &word_lower[..word_lower.len() - suffix.len()];
                    if prefix.len() >= 2 {
                        return Some(format!("{}{}", prefix, replacement));
                    }
                }
            }
        }

        None
    }
}
