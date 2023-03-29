use std::{vec, ops::Index};
use yew_router::agent::RouteRequest::ChangeRoute;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::route::AppRoute;
use bson::{doc, oid::ObjectId};

use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    }
};

use crate::types::var::{
    UserSetting,
    ProjectID,
    ProjectStat,
    MsgErr,
};

#[derive(Properties, Clone)]
pub struct SetProps {
    pub _name:String,
}

pub enum Msg {
    InputName(String), //name
    InputDesc(String), //description
    InputEmail(String), //email
    InputApi(String), //api_key
    InputPlatNotif(String), //bot_type
    InputBotTok(String), //token
    InputGroupChatID(String), //chatid
    IssueCreated,
    IssueUpdated,
    IssueDeleted,
    CommentCreated,
    CommentUpdated,
    CommentDeleted,
    Active_btn,
    Ignore,
    RequestData,
    ResponseError(String),
    DeleteConnector,
    CopyDataSetting(UserSetting),
    Direct,
    UpdateConnector,
    GetProject,
    CopyDataProject(Vec<ProjectID>),
    TriggerProject(usize),
    UpdateValidate,
}

pub struct ConnectorSetting {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    message: String,
    user_setting:UserSetting, 
    _name: String,
    project_id:Vec<ProjectID>,
    fetch_task: Option<FetchTask>,
    project_stat:Vec<ProjectStat>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    msg_err:MsgErr,
}

impl Component for ConnectorSetting {
    type Message = Msg;
    type Properties = SetProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("this is homepage..........");
        Self {   
            message: String::from("initial message"),
            project_stat:vec![],
            project_id:vec![],
            user_setting:UserSetting { 
                name:"".to_string(), 
                description:"".to_string(), 
                email:"".to_string(), 
                api_key:"".to_string(), 
                bot_type:"".to_string(), 
                token: "".to_string(), 
                chatid:"".to_string(),
                active:false,
                event:vec![],
                project:vec![],
            },
            msg_err:MsgErr { 
                header:"".to_string(),
                body:"".to_string(),
            },
            _name:props._name,
            link: link.clone(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            fetch_task:None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                //     input.focus();
                // }
                
                // ConsoleService::info("this is first render homepage.....");
                self.link.send_message(Msg::RequestData);
               
            }
        }
        
        fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

            Msg::DeleteConnector => {
                //FETCHING...

                let request = Request::delete(format!("https://atlassian-connector-api.dev-domain.site/connector/{}", self._name))
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<UserSetting>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));

                        if meta.status.is_success(){
                            Msg::Direct
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg::Direct
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

            Msg::GetProject => {

                    let request = Request::get(format!("https://atlassian-connector-api.dev-domain.site/projects?email={}&api_key={}", self.user_setting.email , self.user_setting.api_key))
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<ProjectID>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        match data {
                            Ok(dataok) => {

                                if status_number == 200 {
                                    Msg::CopyDataProject(dataok)
                                } else {
                                    Msg::ResponseError(String::from("status bukan 200"))
                                }

                            }
                            Err(error) => {
                                // ConsoleService::info("kondisi error dari server mati");
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true

            }

            Msg::UpdateConnector => {

                let mut final_project:Vec<ProjectID> = Vec::new();

                for x in self.project_stat.clone() {
                    if x.status {
                        final_project.push(ProjectID { 
                            id: x.id, 
                            name: x.name
                        })
                    }
                }
                

                let user_setting = UserSetting {
                    name: self.user_setting.name.clone(),
                    description: self.user_setting.description.clone(),
                    api_key: self.user_setting.api_key.clone(),
                    email: self.user_setting.email.clone(),
                    token: self.user_setting.token.clone(),
                    active: self.user_setting.active.clone(),
                    bot_type: self.user_setting.bot_type.clone(),
                    chatid: self.user_setting.chatid.clone(),
                    event: self.user_setting.event.clone(),
                    project: final_project,
                };

                // Fetching
                let request = Request::put(format!("https://atlassian-connector-api.dev-domain.site/connector/{}", self._name))
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .header("Content-Type", "application/json")
                    .body(Json(&user_setting))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));

                        if meta.status.is_success(){
                            Msg::Direct
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg::Direct
                                }
                                Err(error) => {
                                    ConsoleService::info("ignore.");
                                    Msg::ResponseError(error.to_string())
                                }
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true
            }


            Msg::RequestData => {
                //FETCHING...

                let request = Request::get(format!("https://atlassian-connector-api.dev-domain.site/connector/{}", self._name))
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<UserSetting, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        ConsoleService::info(&format!("data response {:?}", &data));
                        

                        match data {
                            Ok(dataok) => {

                                if status_number == 200 {
                                    Msg::CopyDataSetting(dataok)
                                } else {
                                    Msg::ResponseError(String::from("status bukan 200"))
                                }

                            }
                            Err(error) => {
                                // ConsoleService::info("kondisi error dari server mati");
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true
            }
            Msg::InputName(name)=>{
                self.user_setting.name = name;
                ConsoleService::info(&format!("ConnectorName adalah: {:?}", self.user_setting));
                true
            }
            Msg::InputDesc(description)=>{
                self.user_setting.description = description;
                ConsoleService::info(&format!("Description adalah: {:?}", self.user_setting));
                true
            }
            Msg::InputEmail(email)=>{
                self.user_setting.email = email;
                ConsoleService::info(&format!("Email adalah: {:?}", self.user_setting));
                true
            }
            Msg::InputApi(api_key)=>{
                self.user_setting.api_key = api_key;
                ConsoleService::info(&format!("ApiKeys adalah: {:?}", self.user_setting));
                true
            }
            Msg::InputPlatNotif(data) => {
                self.user_setting.bot_type = data;
                ConsoleService::info(&format!("Select Platform Notif: {:?}", self.user_setting));
                true
            }
            Msg::InputBotTok(token)=>{
                self.user_setting.token = token;
                ConsoleService::info(&format!("Bot Token adalah: {:?}", self.user_setting));
                true
            }
            Msg::InputGroupChatID(chatid)=>{
                self.user_setting.chatid = chatid;
                ConsoleService::info(&format!("Group Chat ID adalah: {:?}", self.user_setting));
                true
            }
            Msg::Ignore=>{
                true
            }
            Msg::Direct=> {
                ConsoleService::info(("Direct Jalan"));

                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
                true
            }
            Msg::TriggerProject(index)=>{
                ConsoleService::info("TriggerProject....");
                
                if self.project_stat.get(index).unwrap().status {
                    self.project_stat.get_mut(index).unwrap().status = false
                }else{
                    self.project_stat.get_mut(index).unwrap().status = true
                }

  
                ConsoleService::info(&format!("Issue Created adalah {:?}", self.user_setting));   
                true
            }
            Msg::IssueCreated=>{
                ConsoleService::info("Issue Created....");
                match self.user_setting.event.iter().position(|i| i=="jira:issue_created"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("jira:issue_created".to_string()),
                }    
                ConsoleService::info(&format!("Issue Created adalah {:?}", self.user_setting));   
                true
            }
            Msg::IssueUpdated=>{
                ConsoleService::info("Issue Updated....");
                match self.user_setting.event.iter().position(|i| i=="jira:issue_updated"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("jira:issue_updated".to_string()),
                }    
                ConsoleService::info(&format!("Issue Updated adalah {:?}", self.user_setting));   
                true
            }
            Msg::IssueDeleted=>{
                ConsoleService::info("Issue Deleted....");
                match self.user_setting.event.iter().position(|i| i=="jira:issue_deleted"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("jira:issue_deleted".to_string()),
                }    
                ConsoleService::info(&format!("Issue Deleted adalah {:?}", self.user_setting));   
                true
            }
            Msg::CommentCreated=>{
                ConsoleService::info("Comment Created....");
                match self.user_setting.event.iter().position(|i| i=="comment_created"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("comment_created".to_string()),
                }    
                ConsoleService::info(&format!("Comment Created adalah {:?}", self.user_setting));   
                true
            }
            Msg::CommentDeleted=>{
                ConsoleService::info("Comment Deleted....");
                match self.user_setting.event.iter().position(|i| i=="comment_deleted"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("comment_deleted".to_string()),
                }    
                ConsoleService::info(&format!("Comment Deleted adalah {:?}", self.user_setting));   
                true
            }
            Msg::CommentUpdated=>{
                ConsoleService::info("Comment Updated....");
                match self.user_setting.event.iter().position(|i| i=="comment_updated"){  

                    Some(x) => { 
                        self.user_setting.event.remove(x);
                        
                    },
                    None    => self.user_setting.event.push("comment_updated".to_string()),
                }    
                ConsoleService::info(&format!("Comment Updated adalah {:?}", self.user_setting));   
                true
            }
            Msg::Active_btn=>{
                ConsoleService::info("Active ....");
                if self.user_setting.active == true{  
                    self.user_setting.active = false;
                    ConsoleService::info(&format!("1 adalah {:?}", self.user_setting.active));   
                }else {
                    self.user_setting.active = true;
                    ConsoleService::info(&format!("3 adalah {:?}", self.user_setting.active));
                }

                ConsoleService::info(&format!("Active adalah {:?}", self.user_setting));   
                true
            }
            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                true
            }
            Msg::UpdateValidate => {
                // if self.user_setting.name.is_empty(){
                //    self.msg_err.header = "Error".to_string();
                //    self.msg_err.body = "Name field cannot be empty".to_string();
                // }else{
                //     if self.user_setting.email.is_empty(){
                //         self.msg_err.header = "Error".to_string();
                //         self.msg_err.body = "Email field cannot be empty".to_string();
                //     }else{
                //         if self.user_setting.api_key.is_empty(){
                //             self.msg_err.header = "Error".to_string();
                //             self.msg_err.body = "Api key field cannot be empty".to_string();
                //         }else{
                //             if self.user_setting.bot_type.is_empty(){
                //                 self.msg_err.header = "Error".to_string();
                //                 self.msg_err.body = "Platform Notification field cannot be empty".to_string();
                //             }else{
                //                 if self.user_setting.chatid.is_empty() && self.new_connector.bot_type != "Slack"{
                //                     self.msg_err.header = "Error".to_string();
                //                     self.msg_err.body = "Group Chat ID field cannot be empty".to_string();
                //                 }else{
                //                     if self.user_setting.token.is_empty(){
                //                         self.msg_err.header = "Error".to_string();
                //                         self.msg_err.body = "Bot Token field cannot be empty".to_string();
                //                     }else{
                //                         if self.user_setting.
                //                     }
                //                 }
                //             }
                //         }
                //     }
                // }
                true
            }
            Msg::CopyDataSetting (data) => {
                self.user_setting = data;
                self.link.send_message(Msg::GetProject);
                true
            }
            Msg::CopyDataProject (data) => {
                ConsoleService::info(&format!("All Project {:?}", &data));

                let id_project:Vec<String> = self.user_setting.project.iter().map(|id_|id_.id.clone()).collect();

                if id_project.is_empty() {
                    for x in data {
                        self.project_stat.push(
                            ProjectStat { 
                                id: x.id.clone(),
                                name: x.name.clone(), 
                                status: false
                            }
                        );
                    }
                       
                }else {
                self.project_stat = data.iter().map(|project|{
                        if id_project.contains(&project.id){
                            ProjectStat {
                                id: project.id.clone(),
                                name: project.name.clone(),
                                status: true,
                            }   
                        }else{
                            ProjectStat {
                                id: project.id.clone(),
                                name: project.name.clone(),
                                status: false,
                            }  
                        }
                    }).collect();
                }
                ConsoleService::info(&format!("Selected Project {:?}", self.user_setting.project));
                ConsoleService::info(&format!("Project Stat {:?}", self.project_stat));
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
        type Anchor = RouterAnchor<AppRoute>;

        let mut setpoint=0;

        //Issue Created
        let mut event_issue_created= false;
        if self.user_setting.event.clone().iter().any(|i| i=="jira:issue_created"){
            event_issue_created=true;
        }
        ConsoleService::info(&format!("Event_issue_created: {:?}", event_issue_created));
        
        //Issue Updated
        let mut event_issue_updated= false;
        if self.user_setting.event.clone().iter().any(|i| i=="jira:issue_updated"){
            event_issue_updated=true;
        }
        ConsoleService::info(&format!("Event_issue_updated: {:?}", event_issue_updated));

        //Issue Deleted
        let mut event_issue_deleted= false;
        if self.user_setting.event.clone().iter().any(|i| i=="jira:issue_deleted"){
            event_issue_deleted=true;
        }
        ConsoleService::info(&format!("Event_issue_deleted: {:?}", event_issue_deleted));

        //Comment Created
        let mut event_comment_created= false;
        if self.user_setting.event.clone().iter().any(|i| i=="comment_created"){
            event_comment_created=true;
        }
        ConsoleService::info(&format!("Event_comment_created: {:?}", event_comment_created));

        //Comment Updated
        let mut event_comment_updated= false;
        if self.user_setting.event.clone().iter().any(|i| i=="comment_updated"){
            event_comment_updated=true;
        }
        ConsoleService::info(&format!("Event_comment_updated: {:?}", event_comment_updated));

        //Comment Deleted
        let mut event_comment_deleted= false;
        if self.user_setting.event.clone().iter().any(|i| i=="comment_deleted"){
            event_comment_deleted=true;
        }
        ConsoleService::info(&format!("Event_comment_deleted: {:?}", event_comment_deleted));

        //button
        let mut event_button: bool = false;
        event_button = self.user_setting.active;

        // if self.user_setting.active == true.to_string() {
        //     event_button=true;
        // }else if self.user_setting.active == false.to_string() {
        //     event_button=false;
            
        // }
        ConsoleService::info(&format!("Event_button: {:?}", event_button));
        

        html! {
            <div
            >
                <div 
                    style= "
                        padding-right: 30px;  
                        padding-left: 84%;
                        padding-top:50px;
                    "
                    
                >
                    {
                        if event_button == true{
                            html!{
                                <button 
                                    type="button"
                                    class="btn btn-info mt-4 mb-3"
                                        style="
                                            background: green;
                                            border-color:green;
                                            color:white;
                                            width:60px;
                                        "
                                    
                                    onclick=self.link.callback(|_| Msg::Active_btn)
                                >
                                    {"ON"}
                                </button>
                            }
                        }else{
                            html!{
                                <button 
                                    type="button"
                                    class="btn btn-info mt-4 mb-3"
                                    style="
                                        background:#A73034;
                                        border-color:#A73034;
                                        color:white;
                                        width:60px;
                                    "
                                
                                    onclick=self.link.callback(|_| Msg::Active_btn)
                                >
                                    {"OFF"}
                                </button>
                            }
                        }
                    }

                       
                </div>
                  

                <div class="info"
                    style="
                        text-align:center;
                        font-weight: bold;
                    "
                >
                    <p> {"Basic Information"} </p>
                </div>


                <input type="text" id="" class="form-control p-3 my-2 " placeholder="Connector Name"
                    style="
                        width: 400px;
                        height:30px;
                        margin:auto;
                    "
                    value={self.user_setting.name.clone()}
                    oninput=self.link.callback(|data: InputData| Msg::InputName(data.value))
                />
                 

                <div class="space"
                    style="
                        color:white;
                        height: 5px;
                    "
                >
                    <p> {"s"} </p>
                </div>


                <input type="text" id="" class="form-control p-3 my-2 " placeholder="Description"
                    style="
                        width: 400px;
                        height:30px;
                        margin:auto;
                    "
                    value={self.user_setting.description.clone()}
                    oninput=self.link.callback(|data: InputData| Msg::InputDesc(data.value))
                />
                 

                <div class="space"
                    style="
                        color:white;
                        height: 15px;
                    "
                >
                    <p> {"s"} </p>
                </div>


                <div class="info"
                    style="
                        text-align:center;
                        font-weight: bold
                    "
                >
                    <p> {"Credential Platform"} </p>
                </div>
                  

                <input type="text" id="" class="form-control p-3 my-2 " placeholder="Email"
                    style="
                        width: 400px;
                        height:30px;
                        margin:auto;
                    "
                    value={self.user_setting.email.clone()}
                    oninput=self.link.callback(|data: InputData| Msg::InputEmail(data.value))
                />


                <div class="space"
                    style="
                        color:white;
                        height: 5px;
                    "
                >
                    <p> {"s"} </p>
                </div>


                <input type="password" id="" class="form-control p-3 my-2 " placeholder="API Keys"
                    style="
                        width: 400px;
                        height:30px;
                        margin:auto;
                    "
                    value={self.user_setting.api_key.clone()}
                    oninput=self.link.callback(|data: InputData| Msg::InputApi(data.value))
                />


                <div class="dropdown1" //sini
                    style="
                        color:white;
                        height: 5px;
                    "
                >
                    <div class="dropdown"
                        style="
                            color:#212529;
                            margin:auto;
                        "
                    >

                        <div class="space"
                            style="
                                color:white;
                                height: 10px;
                                margin:auto;
                            "
                        > 
                        </div>
                        

                            <div class="info"
                                style="
                                    text-align:center;
                                    font-weight: bold
                                "
                            >
                                <p> {"Pick Project"} </p>
                                
                                
                                <button
                                    type="button"
                                    class="btn btn-primary"
                                    data-bs-toggle="modal"
                                    data-bs-target="#exampleModal"
                                    style="
                                        width:400px;
                                    "
                                >
                                    {"Pick your project"}
                                </button>


                                <div 
                                    class="modal fade"
                                    id="exampleModal"
                                    tabindex="-1"
                                    aria-labelledby="exampleModalLabel"
                                    aria-hidden="true"
                                >
                                    <div class="modal-dialog modal-dialog-scrollable"
                                    >
                                        <div class="modal-content"
                                        >
                                            <div class="modal-header"
                                            >
                                                <h5 class="modal-tittle">
                                                    <p> {"Project List"} </p>
                                                </h5>

                                                <button 
                                                    type="button"
                                                    class="btn-close"
                                                    data-bs-dismiss="modal"
                                                    aria-label="close"
                                                >
                                                </button>
                                            </div>
                                        
                                        
                                            <div class="modal-body"
                                                style="
                                                    text-align:center;
                                                    margin:auto;
                                                "
                                            >
                                                {                                                
                                                    self.project_stat.iter().enumerate().map(|(index,i)| {
                                                            html! {
                                                                <div class="form-check mb-3" style=" width:400px;"
                                                                >
                                                                    <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                                                    checked={i.status}
                                                                    onclick=self.link.callback(move |_| Msg::TriggerProject(index))
                                                                    />
                                                                    <label class="form-check-label" for="flexCheckDefault"
                                                                    >
                                                                    {&i.name}
                                                                    


                                                                    </label>
                                                                </div>
                                                            }
                                                        }
                                                    ).collect::<Html>() 
                                                }
                                            </div>

                                            <div class="modal-footer"
                                            >
                                                <button
                                                    type="button"
                                                    class="btn btn-secondary"
                                                    data-bs-dismiss="modal"
                                                >
                                                    {"close"}
                                                </button> 
                                                
                                                <button
                                                    type="button"
                                                    class="btn btn-primary"
                                                    data-bs-dismiss="modal"
                                                >
                                                    {"Save changes"}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                    </div>
                        </div>
                          
                      <div class="space"
                      style="
                      color:white;
                      height: 30px;
                      "
                 >
                     <p> {"s"} </p>
                 </div>

                        <div> // atas
                                    <div class="Credential bot"
                                    style="
                                    text-align:center;
                                    height:30px;
                                    "
                            >
                            <p
                                style="
                                color:black;
                                font-weight: bold
                                "
                            > {"Credential Bot"} </p>
                            </div>
                            
                            <form style="text-align:center; padding-top:15px;"
                            >
                                <select class="form-select mb-4" aria-label="Default select example"
                                    style="
                                        margin:auto;
                                        width: 400px;
                                        height:36px;
                                    "
                                    onchange=self.link.callback(|e| {
                                        if let ChangeData::Select(select) = e {
                                            let value = select.value();
                                            Msg::InputPlatNotif(value)
                                        } else {
                                            Msg::InputPlatNotif("No value".to_string())
                                        }
                                    })
                                >   
                                <option selected={
                                    if self.user_setting.bot_type == "Telegram"{true}
                                    else{false} 
                                }>{"Telegram"} </option>
                                <option selected={
                                    if self.user_setting.bot_type == "Slack" {true}
                                    else {false} 
                                }>{"Slack"} </option>
                                    // <option value="Telegram">{"Telegram"}</option>
                                    // <option value="Slack">{"Slack"}</option>

                                   
                                </select>
                            </form>

                            {
                                if self.user_setting.bot_type == "Telegram"{
                                    html!{
                                        <div>
                                            <input type="text" id="Bot_Tok" class="form-control p-3 my-2 " placeholder="Bot Token"
                                                style="
                                                    width: 400px;
                                                    height:30px;
                                                    margin:auto;
                                                "
                                                value={self.user_setting.token.clone()}
                                                oninput=self.link.callback(|data: InputData| Msg::InputBotTok(data.value))
                                            />
                                        
                                            <div class="space"
                                                    style="
                                                    color:white;
                                                    height: 5px;
                                                    "
                                            >
                                                <p> {"s"} </p>
                                            </div>
                                            <input type="text" id="Group_ID" class="form-control p-3 my-2 " placeholder="Group Chat ID"
                                                style="
                                                    width: 400px;
                                                    height:30px;
                                                    margin:auto;
                                                "
                                                value={self.user_setting.chatid.clone()}
                                            oninput=self.link.callback(|data: InputData| Msg::InputGroupChatID(data.value))
                                             />
                                        </div>
                                    }
                                }else{
                                    html!{
                                        <div>
                                            <input type="text" id="Bot_Tok" class="form-control p-3 my-2 " placeholder="Bot Token"
                                                style="
                                                    width: 400px;
                                                    height:30px;
                                                    margin:auto;
                                                "
                                                value={self.user_setting.token.clone()}
                                                oninput=self.link.callback(|data: InputData| Msg::InputBotTok(data.value))
                                            />

                                        </div>
                                    }
                                }
                            }

                            <div class="space"
                                style="
                                color:white;
                                height: 30px;
                                "
                            >
                                <p> {"s"} </p>
                            </div>
                        </div>

                        <div>//atas 2
                            <div>
                                <div style="text-align:center;height:30px;">
                                <p style=" color:black; font-weight: bold; margin=auto;"> {"Notification Setting"} </p> </div>
                                    <div class="check-box"
                                         style="
                                            color:black;
                                            margin:auto;
                                            text-align:center;
                                         "
                                   

                                    >   
                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="issuescreated" id="flexCheckDefault"
                                            checked={event_issue_created} 
                                            onclick=self.link.callback(|_| Msg::IssueCreated)
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Issues Created"}</label>
                                        </div>

                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_issue_updated} 
                                            onclick=self.link.callback(|_| Msg::IssueUpdated)
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Issues Updated"}</label>
                                        </div>

                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_issue_deleted} 
                                            onclick=self.link.callback(|_| Msg::IssueDeleted)  
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Issues Deleted"}</label>
                                        </div>
                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_comment_created} 
                                            onclick=self.link.callback(|_| Msg::CommentCreated)   
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Comment Created"}</label>
                                        </div>
                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_comment_updated} 
                                            onclick=self.link.callback(|_| Msg::CommentUpdated)   
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Comment Updated"}</label>
                                        </div>
                                        <div class="form-check mb-3" style="margin: auto; width:400px;">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                                            checked={event_comment_deleted} 
                                            onclick=self.link.callback(|_| Msg::CommentDeleted) 
                                            />
                                                <label class="form-check-label" for="flexCheckDefault">{"Comment Deleted"}</label>
                                        </div> 
                                    </div>
                            </div>
                        </div> // bawah 2

                                <div class="space"
                                    style="
                                        color:white;
                                        height: 30px;
                                    "
                                >
                                    <p> {"s"} </p>
                                </div>


                        <div class="ButtonSet"
                            style="
                                margin:auto;
                                text-align:center;
                            "
                        >
                                <button type="button" class="vertical-centers"
                                    style="
                                    background-color: #006d90; 
                                    color: white; 
                                    border: 3px solid white;
                                    height: 40px;      
                                    "
                                    onclick=self.link.callback(|_| Msg::UpdateConnector)
                                    // onclick=self.link.callback(|_| Msg::UpdateValidate)
                                >
                                    {"Save Changes"}
                                </button>
                        </div>


                                        <div class="space"
                                            style="
                                                color:white;
                                                height: 15px;
                                            "
                                        >
                                            <p> {"s"} </p>
                                        </div>
                        <div 
                            style="
                                margin:auto;
                                text-align:center;
                            "
                        >
                            
                            <button type="button" class="delete"
                                style="
                                background-color: red; 
                                color: white; 
                                border: 3px solid white;
                                height: 40px;
                                "
                                data-bs-toggle="modal"
                                data-bs-target="#DeleteModal"
                                //onclick=self.link.callback(|_| Msg::DeleteConnector) 
                            >
                                {"Delete Connector"}
                            </button>

                            <div class="modal fade" id="DeleteModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
                            >
                                <div class="modal-dialog"
                                >
                                    <div class="modal-content"
                                    >
                                        <div class="modal-header" style="color:black;"
                                        >
                                            <h5 class="modal-tittle"><p> {"Delete Confirmation"} </p> </h5>
                                            <button 
                                                type="button"
                                                class="btn-close"
                                                data-bs-dismiss="modal"
                                                aria-label="close"
                                            >
                                            </button>
                                        </div>
                                        <div class="modal-body" style="color:black;" >
                                            <p> {"Are you sure you want to delete this connector?"} </p>
                                        </div>
                                        <div class="modal-footer"
                                        >
                                            <button
                                                type="button"
                                                class="btn btn-secondary"
                                                data-bs-dismiss="modal"
                                                onclick=self.link.callback(|_| Msg::Ignore) 
                                            >
                                                {"Cancel"}
                                            </button> 
                                            
                                            <button
                                                type="button"
                                                class="btn btn-primary"
                                                data-bs-dismiss="modal"
                                                onclick=self.link.callback(|_| Msg::DeleteConnector) 
                                            >
                                                {"Delete"}
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                        </div>
                </div>
            </div>
          }
    }
}

// impl ConnectorSetting{
//     fn msg_1(&self)->Html{
//         html!{
//             <div style="background: #A73034; font-family: Alexandria; color: #A73034;" >
//                 <div class="modal fade" id="display_msg" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true"
//                 >
//                     <div class="modal-dialog"
//                     >
//                         <div class="modal-content"
//                         >
//                             <div class="modal-header"
//                             >
//                                 <h5 class="modal-tittle"> <p> {format!("{}!",self.msg_err.header)} </p> </h5>
//                                 <button 
//                                     type="button"
//                                     class="btn-close"
//                                     data-bs-dismiss="modal"
//                                     aria-label="close"
//                                 >
//                                 </button>
//                             </div>
//                             <div class="modal-body" style="color:black;" >
//                                 <p> {format!("{} !",self.msg_err.body)} </p>
//                             </div>
//                             <div class="modal-footer"
//                             >
//                                 <button
//                                     type="button"
//                                     style="
//                                         background:#A73034;
//                                         border-color:#A73034;
//                                         color:white;
//                                         border-radius:15px;
//                                         width: 70px;
//                                         height: 40px; 
//                                     "

//                                     class="btn btn-primary"
//                                     data-bs-dismiss="modal"
//                                     onclick=self.link.callback(|_| Msg::CheckSuccess)
//                                 >
//                                 <p> {"Close"} </p>
//                                 </button>
//                             </div>
//                         </div>
//                     </div>
//                 </div>
//             </div>
//         }
        
//     }
// }