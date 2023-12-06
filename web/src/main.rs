use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use dmlex::{parse, write, Format};

pub enum Msg {
    ChangeInputFormat(String),
    ChangeOutputFormat(String),
    ChangeInput(String),
    ChangeDefaultNamespace(String),
    Ignore,
}

pub struct App {
    input_format : Format,
    output_format : Format,
    input : String,
    output : Result<String, String>,
    default_namespace : String,
}

impl App {
    fn update(&mut self) { 
        if self.input.is_empty() {
            self.output = Ok(String::new());
            return;
        }
        let input_buf = self.input.as_bytes();
        let resource = match parse(input_buf, &self.input_format, &Some(self.default_namespace.clone())) {
            Ok(r) => r,
            Err(e) => {
                self.output = Err(format!("{:?}", e));
                return;
            }
        };
        let mut out = Vec::new();
        self.output = match write(&mut out, &self.output_format, &resource, &Some(self.default_namespace.clone())) {
            Ok(_) => Ok(String::from_utf8(out).unwrap()),
            Err(e) => Err(format!("{:?}", e)),
        };
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            input_format : Format::XML,
            output_format : Format::JSON,
            input: String::from("foo"),
            output: Ok(String::new()),
            default_namespace: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeInputFormat(input) => {
                match input.as_str() {
                    "xml" => self.input_format = Format::XML,
                    "json" => self.input_format = Format::JSON,
                    "rdf" => self.input_format = Format::RDF,
                    _ => self.input_format = Format::XML,
                }
                self.update();
                true
            }
            Msg::ChangeOutputFormat(output) => {
                match output.as_str() {
                    "xml" => self.output_format = Format::XML,
                    "json" => self.output_format = Format::JSON,
                    "rdf" => self.output_format = Format::RDF,
                    _ => self.output_format = Format::XML,
                }
                self.update();
                true
            }
            Msg::ChangeInput(input) => {
                web_sys::console::log_1(&input.clone().into());
                self.input = input;
                self.update();
                true
            }
            Msg::ChangeDefaultNamespace(ns) => {
                self.default_namespace = ns;
                self.update();
                true
            }
            Msg::Ignore => false
        }
    }

    fn view(&self, ctx:&Context<Self>) -> Html {
        let change_input1 = ctx.link().callback(|s| Msg::ChangeInputFormat(s));
        let change_input2 = ctx.link().callback(|s| Msg::ChangeInputFormat(s));
        let change_input3 = ctx.link().callback(|s| Msg::ChangeInputFormat(s));
        let change_output1 = ctx.link().callback(|s| Msg::ChangeOutputFormat(s));
        let change_output2 = ctx.link().callback(|s| Msg::ChangeOutputFormat(s));
        let change_output3 = ctx.link().callback(|s| Msg::ChangeOutputFormat(s));
        let change_input = ctx.link().callback(|s| Msg::ChangeInput(s));
        let change_default_namespace = ctx.link().callback(|s| Msg::ChangeDefaultNamespace(s));
        html!{
            <div class="container mx-auto">
                    <div>
                        <h1 class="text-6xl font-bold">{"DMLEX Converter"}</h1>
                    </div>
                    <div>

                    <h3 class="mt-4 font-semibold text-gray-900">{ "Input Format" }</h3>
                    <ul class="items-center text-sm font-medium text-gray-900 sm:flex">
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="input-xml" type="radio" value="" name="input-format" class="w-4 h-4"
                                checked={self.input_format == Format::XML} onclick={move |_| change_input1.emit("xml".to_string())}/>
                                <label for="input-xml" class="text-gray-900">{ "XML" }</label>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="input-json" type="radio" value="" name="input-format" class="w-4 h-4"
                                checked={self.input_format == Format::JSON} onclick={move |_| change_input2.emit("json".to_string())}/>
                                <label for="input-json" class="text-gray-900 ">{ "JSON" }</label>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="input-rdf" type="radio" value="" name="input-format" class="w-4 h-4"
                                checked={self.input_format == Format::RDF} onclick={move |_| change_input3.emit("rdf".to_string())}/>
                                <label for="input-rdf" class="text-gray-900">{ "RDF" }</label>
                            </div>
                        </li>
                    </ul>

                    </div>

                    <div>

                    <h3 class="mt-4 font-semibold text-gray-900">{ "Output Format" }</h3>
                    <ul class="items-center text-sm font-medium text-gray-900 sm:flex">
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="output-xml" type="radio" value="" name="output-format" class="w-4 h-4"
                                checked={self.output_format == Format::XML} onclick={move |_| change_output1.emit("xml".to_string())}/>
                                <label for="output-xml" class="text-gray-900">{ "XML" }</label>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="output-json" type="radio" value="" name="output-format" class="w-4 h-4"
                                checked={self.output_format == Format::JSON} onclick={move |_| change_output2.emit("json".to_string())}/>
                                <label for="output-json" class="text-gray-900 ">{ "JSON" }</label>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="output-rdf" type="radio" value="" name="output-format" class="w-4 h-4"
                                checked={self.output_format == Format::RDF} onclick={move |_| change_output3.emit("rdf".to_string())}/>
                                <label for="output-rdf" class="text-gray-900">{ "RDF" }</label>
                            </div>
                        </li>
                    </ul>

                    </div>
                    { 
                        if self.input_format == Format::RDF || self.output_format == Format::RDF {
                            html! { <>
                    <h3 class="mt-4 font-semibold text-gray-900">{ "Default Namespace" }</h3>
                    <div class="flex w-full">
                        <div class="w-1/2 m-2">
                            <input id="default-namespace" type="text" class="w-full rounded outline-2 border-2 border-slate-600 p-2"
                            placeholder="Default Namespace" value={self.default_namespace.clone()} oninput={move |e : InputEvent| change_default_namespace.emit(
                                e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value() 
                                ) }/>
                        </div>
                    </div>
                    </>
                            }
                        } else { html! {} }
                    }

                    <div class="flex w-full">
                        <div class="w-1/2 m-2">
                            <textarea id="input_textarea"
                            class="w-full rounded outline-2 border-2 border-slate-600 p-2"
                            rows=10 placeholder="Input" oninput={move |e : InputEvent| change_input.emit(
                                e.target().unwrap().dyn_into::<web_sys::HtmlTextAreaElement>().unwrap().value() 
                                ) }/>
                        </div>
                        <div class="w-1/2 m-2">
                            <div 
                            class={
                                if self.output.is_err() {
                                    classes!["w-full", "rounded", "outline-2", "border-2", "border-red-600", "bg-red-100", "p-2", "whitespace-pre"]
                                } else {
                                    classes!["w-full", "rounded", "outline-2", "border-2", "border-slate-600", "p-2", "whitespace-pre"]
                                }
                            }
                            rows=10 placeholder="Output" readonly=true>{ 
                                match &self.output {
                                    Ok(s) => s,
                                    Err(e) => e,
                                }
                            }</div>
                        </div>
                    </div>
            </div>
        }
    }
}




fn main() {
    yew::Renderer::<App>::new().render();
}
