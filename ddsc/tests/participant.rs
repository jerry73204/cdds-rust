use std::time::Duration;

use anyhow::Result;
use dds::{Durability, History, Participant, QoS, Reliability};

#[test]
fn participant_test() -> Result<()> {
    let qos = {
        let mut qos = QoS::default();
        qos.history(History::KeepAll)
            .durability(Durability::Persistent)
            .reliability(Reliability::Reliable {
                max_blocking_time: Duration::from_millis(1000).into(),
            });
        qos
    };
    let part = Participant::new(None, Some(&qos))?;

    Ok(())
}
