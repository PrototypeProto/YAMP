use iced::widget::{self, container};

struct Playback {
    playback_time: f32,
    music_volume: f32,
}



// The event that occurs, relays information back to app to update
#[derive(Debug, Clone, Copy)]
enum Message {
    TimeChanged(f32),
    VolumeChanged(f32),
}

impl Playback {
    fn new() -> Self {
        // inits counter struct or fetch prev playback time
        Self {playback_time: 0.0, music_volume: 0.0}
    }
    
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        // handles emitted messages
        match message {
            Message::TimeChanged(time_change) => self.playback_time = time_change,
            Message::VolumeChanged(music_volume) => self.music_volume = music_volume,
        }
    
        iced::Task::none()
    }
    
    fn view(&self) -> iced::Element<'_, Message> {
        // create view logic ui
        let column = widget::column![

            widget::container( /* This is the toolbar */
                widget::row![
                    widget::button("File"), 
                    widget::button("Edit"),
                    widget::button("Media"),
                    widget::button("Library"),
                ].spacing(5) /* Adds 5px of space between the buttons above */
            )
                .align_left(iced::Length::Fill)
                .padding(2)
                .width(iced::Length::Fill)
                .height(45)
                ,
            widget::container( /* This is the media and music time slider */
                widget::row![
                    widget::container(
                        widget::row![
                            widget::button("RWD"), /* Rewind/Previous song */
                            widget::button("P/Pl"), /* Pause/Play */
                            widget::button("Skip"), /* Skip */
                            widget::button("Shuffle"), /* Shuffle/Random */
                        ].spacing(6)
                    )
                        .width(iced::Length::FillPortion(2))
                        .padding(5)
                        .center_y(iced::Length::Fill)
                        ,
                    widget::container(
                        widget::slider(0.0..=100.0, self.music_volume, Message::VolumeChanged)
                    )
                        .width(iced::Length::FillPortion(1))
                        .center_y(iced::Length::Fill)
                        .padding(5)
                        ,
                    widget::container(
                        widget::slider(0.0..=100.0, self.playback_time, Message::TimeChanged)
                    )
                        .width(iced::Length::FillPortion(6))
                        .center_y(iced::Length::Fill)
                        .padding(5)
                        ,
                ]
            )
                .height(40)
                .width(iced::Length::Fill)
            ,
            widget::container(
                widget::row![
                    widget::button("Album")
                        .width(iced::Length::FillPortion(1))
                        .height(iced::Length::Fill)
                    ,
                    container(
                        widget::row![
                            widget::button("Playlist Area")
                                .width(iced::Length::FillPortion(1))
                            ,
                            widget::button("Songslist")
                                .width(iced::Length::FillPortion(3))
                            ,
                        ]
                    ).width(iced::Length::FillPortion(3))
                ]
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fill)
            )
                .padding(5)
                ,

            // widget::button("+").on_press(Message::IncrementCount).width(50),
            // widget::text(self.count),
            // widget::button("-").on_press(Message::DecrementCount).width(50)
        ]; //spacing(25)

        widget::container(column)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

}

fn main() -> Result<(), iced::Error> {
    //run the app
    iced::application("Yet Another Music Player", Playback::update, Playback::view).run_with(|| (Playback::new(), iced::Task::none()))
} 
