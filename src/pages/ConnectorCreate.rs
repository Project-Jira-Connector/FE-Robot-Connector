use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::route::AppRoute;
use yew_router::agent::RouteRequest::ChangeRoute;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    }
};

use crate::types::var::{
    NewConnector,
    MsgErr,
};



pub enum Msg {
    InputName(String), // name
    InputDesc(String), //Description
    InputEmail(String), //email
    InputBotTok(String), //token
    InputGroupChatID(String), // chatid
    InputPlatNotif(String), //bot_type
    InputApi(String), //Api_key
    InputActive(String), //Active
    Ignore, 
    Direct, //go to homepage 
    CreateConnector,
    InputErrorMsg(String),
    CreateValidate,
    ErrorBotType(String),
    CheckInput,
    CheckSuccess,
}

pub struct ConnectorCreate {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    new_connector:NewConnector,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    msg_err:MsgErr,
}

impl Component for ConnectorCreate {
    type Message = Msg;
    type Properties = ();

    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Create New Connector Page");
        Self {
            new_connector:NewConnector { 
                name:"".to_string(),
                description:"".to_string(),
                email:"".to_string(),
                api_key:"".to_string(),
                bot_type:"".to_string(),
                active:false,
                token:"".to_string(),
                chatid:"".to_string(),
                project:vec![],
                event:vec![],
            },
            msg_err:MsgErr { 
                header:"".to_string(),
                body:"".to_string(),
            },
            fetch_task: None,
            link: link.clone(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
        }
    }
    
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("this is first render homepage.....");

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputName(name)=>{
                self.new_connector.name = name;
                ConsoleService::info(&format!("ConnectorName adalah: {:?}", self.new_connector));
                true
            }
            Msg::InputDesc(description)=>{
                self.new_connector.description = description;
                ConsoleService::info(&format!("Description adalah: {:?}", self.new_connector));
                true
            }
            Msg::InputEmail(email)=>{
                self.new_connector.email = email;
                ConsoleService::info(&format!("Email adalah: {:?}", self.new_connector));
                true
            }
            Msg::InputApi(data) => {
                self.new_connector.api_key = data;
                ConsoleService::info(&format!("Api Key: {:?}", self.new_connector));
                true
            }
            Msg::InputPlatNotif(data) => {
                self.new_connector.bot_type = data;
                ConsoleService::info(&format!("Select Platform Notif: {:?}", self.new_connector));
                true
            }
            Msg::InputActive(data) => {
                self.new_connector.active = false;
                ConsoleService::info(&format!("Active: {:?}", self.new_connector));
                true
            }
            Msg::InputBotTok(token)=>{
                self.new_connector.token = token;
                ConsoleService::info(&format!("Bot Token adalah: {:?}", self.new_connector));
                true
            }
            Msg::InputGroupChatID(chatid)=>{
                self.new_connector.chatid = chatid;
                ConsoleService::info(&format!("Group Chat ID adalah: {:?}", self.new_connector));
                true
            }
            //test
            Msg::CreateConnector => {
                
                let new_connector = NewConnector {
                    name: self.new_connector.name.clone(),
                    description: self.new_connector.description.clone(),
                    api_key: self.new_connector.api_key.clone(),
                    email: self.new_connector.email.clone(),
                    token: self.new_connector.token.clone(),
                    active: self.new_connector.active.clone(),
                    bot_type: self.new_connector.bot_type.clone(),
                    chatid: self.new_connector.chatid.clone(),
                    event: self.new_connector.event.clone(),
                    project: self.new_connector.project.clone(),
                };


                let request = Request::post("https://atlassian-connector-api.dev-domain.site/connector")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .header("Content-Type", "application/json")
                    .body(Json(&new_connector))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("data is {:?}", data));
                        ConsoleService::info(&format!("status is {:?}", status_number));

                        if meta.status.is_success(){
                            Msg::CheckInput
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg::InputErrorMsg(dataok)
                                }
                                Err(error) => {
                                    ConsoleService::info("ignore.");
                                    Msg::Ignore
                                }
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);


                true
            }
            Msg::Direct=> {
                ConsoleService::info(("Direct Jalan"));

                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
                true
            }
            Msg::Ignore => {
                false
            }
            Msg::InputErrorMsg(dataok) => {
                self.msg_err.header = "Error".to_string();
                self.msg_err.body = dataok;
                true
            }
            Msg::ErrorBotType(data) => {

                true
            }
            Msg::CheckInput => {
                if self.msg_err.body.is_empty(){
                    self.msg_err.header = "Success".to_string();
                    self.msg_err.body = "You have created a new connector".to_string();
                }else{
                    self.link.send_message(Msg::Ignore);
                }
                true
            }
            Msg::CreateValidate => {
                if self.new_connector.name.is_empty(){
                   self.msg_err.header = "Error".to_string();
                   self.msg_err.body = "Name field cannot be empty".to_string();
                }else{
                    if self.new_connector.email.is_empty(){
                        self.msg_err.header = "Error".to_string();
                        self.msg_err.body = "Email field cannot be empty".to_string();
                    }else{
                        if self.new_connector.api_key.is_empty(){
                            self.msg_err.header = "Error".to_string();
                            self.msg_err.body = "Api key field cannot be empty".to_string();
                        }else{
                            if self.new_connector.bot_type.is_empty(){
                                self.msg_err.header = "Error".to_string();
                                self.msg_err.body = "Platform Notification field cannot be empty".to_string();
                            }else{
                                if self.new_connector.chatid.is_empty() && self.new_connector.bot_type != "Slack"{
                                    self.msg_err.header = "Error".to_string();
                                    self.msg_err.body = "Chat ID field cannot be empty".to_string();
                                }else{
                                    if self.new_connector.token.is_empty(){
                                        self.msg_err.header = "Error".to_string();
                                        self.msg_err.body = "Bot Token field cannot be empty".to_string();
                                    }else{
                                        if self.new_connector.name.ends_with(" "){
                                            self.msg_err.header = "Error".to_string();
                                            self.msg_err.body = "Name field cannot end with a space".to_string();
                                        }else{
                                            self.msg_err.body = "".to_string();
                                            ConsoleService::info(&format!("msg err body {}", self.msg_err.body));
                                            self.link.send_message(Msg::CreateConnector);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                true
            }
            Msg::CheckSuccess => {           
                if self.msg_err.header == "Success"{
                    self.link.send_message(Msg::Direct)
                }else{
                    self.link.send_message(Msg::Ignore)
                }                 

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="base-form">
                <div class="create-connector">
                    <h5>{"Create Connector Form"}</h5>

                    <div class="input-group" style=" margin: auto; width: 400px">
                        <input type="text"  id="emailInput" class="form-control p-3 my-2 " placeholder="Connector Name"
                        style="
                            width: 400px;
                            height:30px;
                            margin:auto;
                        "
                            value={self.new_connector.name.clone()}
                            oninput=self.link.callback(|data: InputData| Msg::InputName(data.value))
                            
                        />
                    </div>

                    <div class="input-group" style=" margin: auto; width: 400px">
                        <input type="text" id="emailInput" class="form-control p-3 my-2 " placeholder="Email"
                        style="
                            height:30px;
                            margin:auto;
                        "
                            value={self.new_connector.email.clone()}
                            oninput=self.link.callback(|data: InputData| Msg::InputEmail(data.value))
                        />
                    </div>

                    <div class="input-group" style=" margin: auto; width: 400px">
                        <input type="password" id="emailInput" class="form-control p-3 my-2 " placeholder="Api Keys"
                        style="
                            width: 400px;
                            height:30px;
                            margin:auto;
                            margin-top:150px;
                        "
                            value={self.new_connector.api_key.clone()}
                            oninput=self.link.callback(|data: InputData| Msg::InputApi(data.value))
                        />
                    </div>  

                    //Platform Notif
                    <div class="dropdown mt-2 mb-2"
                    style="
                        color:#212529;
                        margin:auto;
                        margin-top:150px;
                    "
                    >
                        <form style="text-align:center;"
                        >
                            <select class="form-select"
                                aria-label="Default select example"
                                    style="
                                        margin:auto;
                                        width: 400px;
                                        height:36px;
                                        margin-top:5px;
                                    "
                                    required=true
                                    onchange=self.link.callback(|e| {
                                        if let ChangeData::Select(select) = e {
                                        
                                            let value = select.value();
                                            Msg::InputPlatNotif(value)
                                        } else {
                                            Msg::InputPlatNotif("No value".to_string())
                                        }
                                    })
                            >
                                <option value="" disabled=true  >{"Platform Notification"}</option>
                                <option value="Telegram">{"Telegram"}</option>
                                <option value="Slack">{"Slack"}</option>
                            </select>
                        </form>
                    </div>
                
                

                                
                    { 
                        if self.new_connector.bot_type.eq("Telegram"){
                            html!{
                                <div>

                                    <div class="input-group" style=" margin: auto; width: 400px">
                                        <input type="text" id="emailInput" class="form-control p-3 my-2 " placeholder="Chat ID"
                                        style="
                                            height:30px;
                                            margin:auto;       
                                        "
                                        value={self.new_connector.chatid.clone()}
                                        oninput=self.link.callback(|data: InputData| Msg::InputGroupChatID(data.value))
                                        />
                                    </div>

                                    <div class="input-group" style=" margin: auto; width: 400px">
                                        <input type="text" id="emailInput" class="form-control p-3 my-2 " placeholder="Bot Token"
                                        style="
                                            height:30px; 
                                            margin:auto;
                                        "
                                        value={self.new_connector.token.clone()}
                                        oninput=self.link.callback(|data: InputData| Msg::InputBotTok(data.value))
                                        />
                                    </div>

                                </div>
                            }
                        }else if self.new_connector.bot_type.eq("Slack"){
                            html!{
                                <div>
                                    <div class="input-group" style=" margin: auto; width: 400px">
                                        <input type="text" id="emailInput" class="form-control p-3 my-2 " placeholder="Bot Token"
                                        style="
                                            height:30px; 
                                            margin:auto;
                                        "
                                        value={self.new_connector.token.clone()}
                                        oninput=self.link.callback(|data: InputData| Msg::InputBotTok(data.value))
                                        />
                                    </div>
                                </div>
                            }
                        }else{html!{}}
                        
                    }
                        
                            
                
                    <div style=" text-align:center;"
                    >
                        <button type="button" class="home"
                                style="
                                background:#A73034;
                                border-color:#A73034;
                                color:white;
                                border-radius:15px;
                                height: 40px;
                                margin-top:15px;
                                
                            " 
                            data-bs-toggle="modal"
                            data-bs-target="#display_msg"
                            onclick=self.link.callback(|_| Msg::CreateValidate)
                        >
                            {"Create new Connector"}
                        </button>
                    </div>

                          
        
                    
                </div>
                    {self.msg_1()}      
            </div>
          
        }
    }
}

impl ConnectorCreate{
    fn msg_1(&self)->Html{
        html!{
            <div style="background: #A73034; font-family: Alexandria; color: #A73034;" >
                <div class="modal fade" id="display_msg" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
                >
                    <div class="modal-dialog"
                    >
                        <div class="modal-content"
                        >
                            <div class="modal-header"
                            >
                                <h5 class="modal-tittle"> <p> {format!("{} !",self.msg_err.header)} </p> </h5>
                                <button 
                                    type="button"
                                    class="btn-close"
                                    data-bs-dismiss="modal"
                                    aria-label="close"
                                    onclick=self.link.callback(|_| Msg::CheckSuccess)
                                >
                                </button>
                            </div>
                            <div class="modal-body" style="color:black;" >
                                <p> {format!("{} !",self.msg_err.body)} </p>
                            </div>
                            <div class="modal-footer"
                            >
                                <button
                                    type="button"
                                    style="
                                        background:#A73034;
                                        border-color:#A73034;
                                        color:white;
                                        border-radius:15px;
                                        width: 70px;
                                        height: 40px; 
                                    "

                                    class="btn btn-primary"
                                    data-bs-dismiss="modal"
                                    onclick=self.link.callback(|_| Msg::CheckSuccess)
                                >
                                <p> {"Close"} </p>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
        
    }
}
