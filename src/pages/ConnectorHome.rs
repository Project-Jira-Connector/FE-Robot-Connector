use std::{vec, ops::Index};

use yew::{prelude::*, callback};
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
    GetConnector,
};

pub enum Msg {
    Ignore,
    RequestData,
    GetData(Vec<GetConnector>),
}

pub struct ConnectorHome {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    homepage: Vec<GetConnector>,
    

}

impl Component for ConnectorHome {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("this is homepage..........");
        Self {
            homepage:vec![],
            fetch_task: None,
            link: link.clone(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),

        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // ConsoleService::info("this is first render homepage.....");
            self.link.send_message(Msg::RequestData)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore=>{
                false
            }
            Msg::RequestData => {

              let request = Request::get("https://atlassian-connector-api.dev-domain.site/connector")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<GetConnector>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();
                        match data {
                            Ok(dataok) => {
                                Msg::GetData(dataok)
                            }
                            Err(error) => {
                                Msg::Ignore
                            }
                        }
                    });
                    // ConsoleService::info(&format!("get data {:?}",data));
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.homepage = data;
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
            <div
                style="
                    border:none;
                "
            >
                <div
                    style="
                        background: #E3E8ED; 
                        position: absolute;
                        padding-top: 95px;
                        right: 0;
                        left: 0;
                        overflow: auto;
                        height: 100%;
                        border:none;
                    "
                >    
                    <div style="padding-bottom:30px;"> //button
                        <button type="button" style=" margin-left:87%; background:#A73034; border-color:#A73034;  color:white;  border-radius:15px; height: 40px;"> 
                            <Anchor route=AppRoute::ConnectorCreate> 
                                {"Create new Connector"}  
                            </Anchor>
                        </button>
                    </div>

                    {self.view_index_data()}
                </div>
            </div>
        }
    }
}

impl ConnectorHome {
    fn view_index_data(&self) -> Vec<Html> {
        type Anchor = RouterAnchor<AppRoute>;
        self.homepage.iter().map(|card|{
            ConsoleService::info(&format!("Name adalah {:?}",card.name.to_string()));
                html!{

                        <div class="card mt-4 mb-2"
                            style="
                                text-decoration:none;
                                background: white;
                                width:1200px;
                                margin:auto;
                                border:none;
                            "
                        >
                            <Anchor route=AppRoute::ConnectorSetting { _name:card.name.to_string()}>
                                <div class="card-body" style="color: gray;">
                                    <h4 class="card-title" 
                                        style="color: black;"
                                    >
                                        {&card.name} 
                                    </h4>

                                    <h6 class="card-title"
                                    >
                                        {&card.bot_type}
                                    </h6>
                                    {
                                        if card.active ==  true{
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

                                </div>
                            </Anchor>
                        </div>
                    
                }
        }).collect()          
    }
}
