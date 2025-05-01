use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use std::env;

pub struct Terminal {
    theme: ColorfulTheme,
}

pub enum ModuleType {
    Lib,
    Bin,
}

pub struct ModuleToCreate {
    pub name: String,
    pub module_type: ModuleType,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    pub fn determine_root_folder_name(&self) -> dialoguer::Result<String> {
        let args: Vec<String> = env::args().collect();
        let root_name = if args.len() > 2 {
            args[2].clone()
        } else {
            Input::with_theme(&self.theme)
                .with_prompt("Root folder name:")
                .interact_text()?
        };

        Ok(root_name)
    }

    pub fn determine_modules_for_workspace(&self) -> dialoguer::Result<Vec<ModuleToCreate>> {
        let mut members = Vec::new();

        loop {
            let add_module = Confirm::with_theme(&self.theme)
                .with_prompt("Add a module (crate)?")
                .default(true)
                .interact()?;

            if !add_module {
                return Ok(members);
            }

            let name: String = Input::with_theme(&self.theme)
                .with_prompt("Name:")
                .interact_text()?;

            let module_type_selection = Select::with_theme(&self.theme)
                .with_prompt("Type:")
                .default(0)
                .item("Library")
                .item("Binary")
                .interact()?;
            let module_type = match module_type_selection {
                0 => ModuleType::Lib,
                1 => ModuleType::Bin,
                _ => ModuleType::Lib,
            };

            members.push(ModuleToCreate { name, module_type });
        }
    }
}
