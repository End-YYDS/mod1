use plugin_core::plugin_api::Plugin;
use plugin_core::plugin_macro::*;

#[plugin_entry]
#[plugin_exit]
#[derive(Debug)]
pub struct Mod1;
const NAME: &str = "Mod1";
const VERSION: &str = "0.1.0";
const DESCRIPTION: &str = "This is a mod1 plugin";

impl Plugin for Mod1 {
    fn name(&self) -> &str {
        NAME
    }

    fn version(&self) -> &str {
        VERSION
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn execute(&self, input: &str) -> String {
        format!("Executing with input: {}", input)
    }

    fn unload()
    where
        Self: Sized,
    {
        println!("Unloading Mod1 plugin");
    }

    fn load() -> Box<dyn Plugin>
    where
        Self: Sized,
    {
        println!("[{}]: Loading...", NAME);
        Box::new(Mod1)
    }
}
