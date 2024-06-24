use clap::ValueEnum;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Hash, Eq, PartialEq, Debug, Clone, ValueEnum, Copy)]
pub enum Labels {
    #[clap(alias = "pw3")]
    ProductWeb3,
    #[clap(alias = "bdo")]
    BlockchainDevOps,
    #[clap(alias = "ml")]
    MachineLearning,
    #[clap(alias = "w1")]
    Web1,
    #[clap(alias = "iam")]
    IAMarketing,
}

impl Display for Labels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label_str = match self {
            Labels::ProductWeb3 => "Product Web3",
            Labels::BlockchainDevOps => "Blockchain DevOps",
            Labels::MachineLearning => "Machine Learning",
            Labels::Web1 => "Web 1",
            Labels::IAMarketing => "IA Marketing",
        };
        write!(f, "{}", label_str)
    }
}

impl Labels {
    const IDS: &'static [&'static str] = &[
        "6633a8093462cf3f04d83047", // ProductWeb3
        "6633a612abab8af0d5ef1f8d", // BlockchainDevOps
        "6633a8187180a66a1bdffb51", // MachineLearning
        "6633a5a8b5ccb9f6c72301b9", // Web1
        "6679de475bd63ac0ebc0bfe2", // IAMarketing
    ];

    const COLORS: &'static [&'static str] = &[
        "yellow", // ProductWeb3
        "red",    // BlockchainDevOps
        "purple", // MachineLearning
        "green",  // Web1
        "blue",   // IAMarketing
    ];

    pub fn get_id(&self) -> &str {
        Labels::IDS[*self as usize]
    }

    pub fn get_color(&self) -> &str {
        Labels::COLORS[*self as usize]
    }

    pub fn from_id(id: &str) -> Labels {
        match Labels::IDS.iter().position(|&i| i == id) {
            Some(i) => match i {
                0 => Labels::ProductWeb3,
                1 => Labels::BlockchainDevOps,
                2 => Labels::MachineLearning,
                3 => Labels::Web1,
                4 => Labels::IAMarketing,
                _ => unreachable!(),
            },
            None => panic!("label {} is not a valid", id),
        }
    }

    pub fn get_color_by_id(id: &str) -> String {
        Labels::from_id(id).get_color().to_owned()
    }
}
