use std::sync::Mutex;
use windows::core::*;
use windows::Win32::UI::TextServices::*;

pub enum EditCommand {
    InsertText(String),
    ReplaceText(String),
}

#[implement(ITfEditSession)]
pub struct PtBrEditSession {
    context: ITfContext,
    command: Mutex<Option<EditCommand>>,
}

impl PtBrEditSession {
    pub fn new(context: ITfContext, command: EditCommand) -> Self {
        Self {
            context,
            command: Mutex::new(Some(command)),
        }
    }
}

impl ITfEditSession_Impl for PtBrEditSession {
    fn DoEditSession(&self, ec: u32) -> Result<()> {
        let command = self.command.lock().unwrap().take();
        if let Some(command) = command {
            unsafe {
                match command {
                    EditCommand::InsertText(text) => {
                        let mut fetched = 0;
                        let mut selections = [TF_SELECTION::default()];

                        if self
                            .context
                            .GetSelection(ec, TF_DEFAULT_SELECTION, &mut selections, &mut fetched)
                            .is_ok()
                            && fetched > 0
                        {
                            let selection = &selections[0];
                            // Usando a sugestão do compilador para ManuallyDrop<Option<ITfRange>>
                            if let Some(range) = &*selection.range {
                                let utf16_text: Vec<u16> = text.encode_utf16().collect();
                                range.SetText(ec, 0, &utf16_text)?;
                                range.Collapse(ec, TF_ANCHOR_END)?;
                                self.context.SetSelection(ec, &selections)?;
                            }
                        }
                    }
                    EditCommand::ReplaceText(new_text) => {
                        let mut fetched = 0;
                        let mut selections = [TF_SELECTION::default()];

                        if self
                            .context
                            .GetSelection(ec, TF_DEFAULT_SELECTION, &mut selections, &mut fetched)
                            .is_ok()
                            && fetched > 0
                        {
                            let selection = &selections[0];
                            if let Some(range) = &*selection.range {
                                // Para o scaffold, vamos apenas setar o texto no range atual.
                                // Em uma implementação real, o range deveria cobrir a palavra a ser substituída.
                                let utf16_text: Vec<u16> = new_text.encode_utf16().collect();
                                range.SetText(ec, 0, &utf16_text)?;
                                range.Collapse(ec, TF_ANCHOR_END)?;
                                self.context.SetSelection(ec, &selections)?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
