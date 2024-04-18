#![allow(unused)]

use godot::engine::GDScript;
use godot::prelude::*;


struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

//MUST be a tool node for this to work inside the editor
#[derive(GodotClass)]
#[class(base=Node, init, tool)]
pub struct GDScriptTester {
	base: Base<Node>,
}

#[godot_api]
impl INode for GDScriptTester {
	fn ready(&mut self) {
		let source = GDScriptGenerator::new("GDScriptTester", true)
			.add_signal("TestSignalA")
			.add_signal("TestSignalB")
			.add_export("TestFloat", "float")
			.build();
		
		let mut script = GDScript::new_gd();
		script.set_source_code(source.into());
		script.reload();
		self.base_mut().set_script(script.to_variant());
	}
}



//This "generator" takes a bunch of &str's and outputs a very basic GDScript source code
//Can be extended to include all kinds of stuff as long as it outputs valid GDScript syntax
pub struct GDScriptGenerator<'a> {
	tool_script: bool,
	parent_class: &'a str,
	signals: Vec<&'a str>,
	exports: Vec<(&'a str, &'a str)>,
}
impl<'a> GDScriptGenerator<'a> {
	pub fn new(parent_class: &'a str, tool_script: bool) -> Self {
		Self{ tool_script, parent_class, signals: vec![], exports: vec![] }
	}
	
	pub fn add_signal(mut self, signal_name: &'a str) -> Self {
		self.signals.push(signal_name);
		self
	}
	
	pub fn add_export(mut self, export_name: &'a str, export_type: &'a str) -> Self {
		self.exports.push((export_name, export_type));
		self
	}
	
	pub fn build(mut self) -> String {
		let mut source_code: String = "".into();
		if self.tool_script {
			source_code += "@tool\n";
		}
		
		source_code += "extends ";
		source_code += self.parent_class;
		source_code += "\n\n";
		
		for signal_name in self.signals {
			source_code += "signal ";
			source_code += signal_name;
			source_code += "\n";
		}
		
		for (export_name, export_type) in self.exports {
			source_code += "@export var ";
			source_code += export_name;
			source_code += ": ";
			source_code += export_type;
			source_code += "\n";
		}
		
		source_code
	}
	
}
