use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;

pub enum Msg {
    ChangeInputFormat(String),
    ChangeOutputFormat(String),
    ChangeInput(String),
    Ignore,
}

pub struct App {
    input_format : String,
    output_format : String,
    input : String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            input_format : String::from("xml"),
            output_format : String::from("json"),
            input: String::from("foo"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeInputFormat(input) => {
                self.input_format = input;
                true
            }
            Msg::ChangeOutputFormat(output) => {
                self.output_format = output;
                true
            }
            Msg::ChangeInput(input) => {
                web_sys::console::log_1(&input.clone().into());
                self.input = input;
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
        html!{
            <div class="container mx-auto">
                    <div>
                        <h1 class="text-6xl font-bold">{"DMLEX Converter"}</h1>
                    </div>
                    <div>

                    <h3 class="mb-4 font-semibold text-gray-900">{ "Input Format" }</h3>
                    <ul class="items-center text-sm font-medium text-gray-900 sm:flex">
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="input-xml" type="radio" value="" name="input-format" class="w-4 h-4"
                                checked={self.input_format == "xml"} onclick={move |_| change_input1.emit("xml".to_string())}/>
                                <label for="input-xml" class="text-gray-900">{ "XML" }</label>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="input-json" type="radio" value="" name="input-format" class="w-4 h-4"
                                checked={self.input_format == "json"} onclick={move |_| change_input2.emit("json".to_string())}/>
                                <label for="input-json" class="text-gray-900 ">{ "JSON" }</label>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="input-rdf" type="radio" value="" name="input-format" class="w-4 h-4"
                                checked={self.input_format == "rdf"} onclick={move |_| change_input3.emit("rdf".to_string())}/>
                                <label for="input-rdf" class="text-gray-900">{ "RDF" }</label>
                            </div>
                        </li>
                    </ul>

                    </div>

                    <div>

                    <h3 class="mb-4 font-semibold text-gray-900">{ "Output Format" }</h3>
                    <ul class="items-center text-sm font-medium text-gray-900 sm:flex">
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="output-xml" type="radio" value="" name="output-format" class="w-4 h-4"
                                checked={self.output_format == "xml"} onclick={move |_| change_output1.emit("xml".to_string())}/>
                                <label for="output-xml" class="text-gray-900">{ "XML" }</label>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="output-json" type="radio" value="" name="output-format" class="w-4 h-4"
                                checked={self.output_format == "json"} onclick={move |_| change_output2.emit("json".to_string())}/>
                                <label for="output-json" class="text-gray-900 ">{ "JSON" }</label>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center ps-3">
                                <input id="output-rdf" type="radio" value="" name="output-format" class="w-4 h-4"
                                checked={self.output_format == "rdf"} onclick={move |_| change_output3.emit("rdf".to_string())}/>
                                <label for="output-rdf" class="text-gray-900">{ "RDF" }</label>
                            </div>
                        </li>
                    </ul>

                    </div>

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
                            class="w-full rounded outline-2 border-2 border-slate-600 p-2"
                            rows=10 placeholder="Output" readonly=true>{ self.input.clone() }</div>
                        </div>
                    </div>
            </div>
        }
    }
}




fn main() {
    yew::Renderer::<App>::new().render();
}
