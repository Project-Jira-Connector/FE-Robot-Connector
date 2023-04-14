use std::collections::HashMap;

use yew::prelude::*;

// use crate::pages::content::Content;

use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde::{
    Deserialize,
    Serialize,
};
// use serde_json::{
//     json!
// };
// use crate::types::var::{
//     Schedule,
//     SchedulesData,
// };
use yew_router::prelude::*;
use crate::router::route::AppRoute;
use crate::types::var::{
    UserAccount,
};

pub enum Msg {
    AddOne,
    InputText(String),
    RequestData,
    GetData(Vec<UsersData>),
    ResponseError(String),
    FetchId(String)
}

#[derive(Properties, Clone)]
pub struct PageInput {
    username: String,
    status: String,
}

use crate::types::var::{
    UsersData,
    ProjectList
};

pub struct RobotProject {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    project: Vec<UsersData>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
    value: i64,
    message: String,
    id: String,
}

impl Component for RobotProject {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("RobotProject");
        Self {
            link,
            value: 0,
            message: String::from("Initial Message"),
            project: vec![],
            fetch_task: None,
            error: None,
            id: String::from("")
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestData)
        }
        
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestData => {


                // FETCHING....

                let request = Request::get("https://atlassian-robot-api.dev-domain.site/robots")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<UsersData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        // ConsoleService::info(&format!("data response {:?}",));
                        ConsoleService::info(&format!("Status is{:?}", data));
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                if status_number == 302 {
                                    Msg::GetData(dataok)
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
                let userdata = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(userdata);


                true
            }
            Msg::GetData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.project = data;
                true
            }
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::InputText(data) => {
                self.message = data;
                true
            }
            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                self.error = Some(text);
                true
            }
            Msg::FetchId(id)=>{
                self.id = id;
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

        html! {
            <div class="base-projects">
                <div class="projects">
                    <div class="btn">
                        <Anchor route=AppRoute::RobotCreate>
                            <button type="button" class="btn-create">
                                {"Create Bot"}
                            </button>
                        </Anchor>
                    </div>
                //     <div>
                //     <button
                //         class="badge rounded-pill bg-primary"
                //         onclick=self.link.callback(|_| Msg::RequestData)
                //     >
                //         { "Get More Data" }
                //     </button>
                // </div>
                
                {self.view_index_data()}
                    </div>
                </div>
        }
    }
}

impl RobotProject{
    fn view_index_data(&self) -> Vec<Html> {
        self.project.iter().map(|card|{
            type Anchor = RouterAnchor<AppRoute>;
                    html!{
                        
                        <div class="card mt-4 mb-2"
                        style="
                        width: 1200px;
                        margin: auto;
                        ">
                        <Anchor route=AppRoute::RobotInput {idProject : card.id.oid.clone()}>
                            <div class="card-body" style="color: gray;">
                                <h4 class="card-title" style="color: black;">
                                    {&card.name}
                                </h4>
                                <h6 class="card-title">
                                    {&card.description}
                                </h6>
                                <div class="d-flex align-items-stretch">
                                {
                                    if card.active.to_string().contains("true"){
                                        html!{
                                            <span class="badge bg-success">
                                                {"Active"}
                                                
                                            </span>
                                        }
                                    } else{
                                        html!{
                                            <span class="badge bg-danger">
                                            
                                                {"Deactive"}
                                                
                                            </span>
                                        }
                                    }
                                    
                                } 
                                    {
                                        if card.created.is_some(){
                                            html!{
                                                <span class="badge bg-secondary" style="margin: 0 10px 0;">{format!("Created : {}",&card.created.unwrap().format("%a, %d %b %Y %H:%M:%S").to_string())}</span>
                                            }
                                        }else{
                                            html!{

                                            }
                                        }
                                    }
                                    {
                                        if card.modified.is_some(){
                                            html!{
                                                <span class="badge bg-secondary">{format!("Last Updated : {}",&card.modified.unwrap().format("%a, %d %b %Y %H:%M:%S").to_string())}</span>
                                            }   
                                        }else{
                                            html!{

                                            }
                                        }
                                    }
                                </div>        
                                <div>
                                </div>
                            </div>   
                        </Anchor>
                        </div>
                    }
        }).collect()
    }
}

//push