mod systems;
mod components;
mod event;
mod resources;


pub struct TitlePlugin<T> {
    running_state: T,
}

