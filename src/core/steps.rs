use clap::ValueEnum;
use serde::Deserialize;

#[derive(Deserialize, Hash, Eq, PartialEq, Debug, Clone, ValueEnum, Copy)]
pub enum Steps {
    Backlog,
    PreProduction,
    VisualIdentity,
    Recording,
    Review,
    Editing,
    Publish,
    Completed,
}

impl Steps {
    const IDS: &'static [&'static str] = &[
        "6633bf10887fb53e55941192", // Backlog
        "6633a2169d3a4098e4adcd18", // PreProduction
        "6633a20b86f043945dfaed69", // VisualIdentity
        "6633a227d4871d117fa5fd87", // Recording
        "6633a2399343322d437204b7", // Review
        "6633bf1ce15c02de31b3797d", // Editing
        "6633a24310b855a087d3713a", // Publish
        "6633bf98240d7b4e3ec3b4bc", // Completed
    ];

    pub fn get_id(&self) -> &str {
        Steps::IDS[*self as usize]
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Steps::IDS.iter().position(|&i| i == id).map(|i| match i {
            0 => Steps::Backlog,
            1 => Steps::PreProduction,
            2 => Steps::VisualIdentity,
            3 => Steps::Recording,
            4 => Steps::Review,
            5 => Steps::Editing,
            6 => Steps::Publish,
            7 => Steps::Completed,
            _ => unreachable!(),
        })
    }
}
