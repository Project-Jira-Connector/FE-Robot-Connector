use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::route::AppRoute;



pub enum Msg {
    AddOne,

}


pub struct NavtopConnector {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for NavtopConnector {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
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
            <div class="header">       
            <input type="checkbox" class="openSidebarMenu" id="openSidebarMenu"/>
                <label for="openSidebarMenu" class="sidebarIconToggle">
                    <div class="spinner diagonal part-1"></div>
                    <div class="spinner horizontal"></div>
                    <div class="spinner diagonal part-2"></div>
                </label>
            <div class="sidebar" id="sidebarMenu">
                <ul class="sidebarMenuInner">
                    <li>{"Digital Business"} <span>{"Web Development"}</span></li>

                    //Robot
                    <li class="sidebarDrop"><a>{"Robot Automation"}</a>
                        <ul class="serviceProjects">
                            <li>
                                <Anchor route=AppRoute::RobotHome>
                                            {"Create"}
                                </Anchor>
                            </li>
                        </ul>
                        <ul class="serviceProjects">
                            <li>
                                <Anchor route=AppRoute::RobotProject>
                                            {"Project"}
                                </Anchor>
                            </li>
                        </ul>
                    </li>

                    //Connector
                    <li class="sidebarDrop"><a>{"Atlassian Connector"}</a>
                        <ul class="serviceProjects">
                            <li>
                                <Anchor route=AppRoute::ConnectorHome>
                                    {"Homepage"}
                                </Anchor>
                            </li>
                        </ul>
                        <ul class="serviceProjects">
                            <li>
                                <Anchor route=AppRoute::ConnectorCreate>
                                    {"Create New Connector"}
                                </Anchor>
                            </li>
                        </ul>
                    </li>
                    
                </ul>
            </div>
        <h5 class="new-navbar">{"Digital Business"}</h5>
        </div>
        }
    }
}
