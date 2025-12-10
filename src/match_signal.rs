#[derive(Debug)]
pub struct HappenedTest {
    pub sender: String,
}

impl arg::AppendAll for HappenedTest {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.sender, i);
    }
}

impl arg::ReadAll for HappenedTest {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(HappenedTest { sender: i.read()? })
    }
}

impl dbus::message::SignalArgs for HappenedTest {
    const NAME: &'static str = "HelloHappened";
    const INTERFACE: &'static str = "com.example.dbustest";
}

// ======== //
use std::{error::Error, time::Duration};
use dbus::{Message, arg, blocking::Connection};

pub fn match_signal() -> Result<(), Box<dyn Error>> {
    let c = Connection::new_session()?;

    {
      let proxy = c.with_proxy("com.example.dbustest", "/hello", Duration::from_millis(3000));

      let _id = proxy.match_signal(|h: HappenedTest, _: &Connection, _: &Message| {
        println!("Hello happened with sender: {}", h.sender);
        true
      });

      let (s,): (String,) = proxy.method_call("com.example.dbustest", "Hello", ("match siganl example",))?;
      println!("{}", s);
    }

    loop {
        c.process(Duration::from_millis(10000))?;
    }
}
