use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

pub struct WorkerState{

    _worker_handle : Option<JoinHandle<()>>,
    _sender :  Option<Sender<i32>>,
    _out_pin : Option<OutputPin>,
}

impl WorkerState{

    pub fn new(single_pin : u8) -> Result<WorkerState, Box<dyn Error>> {
        let io = Gpio::new()?;
        let pin = io.get(single_pin)?;
        Ok(WorkerState{
            _worker_handle : Option::None,
            _sender : None,
            _out_pin : Some(pin.into_output()),
        })
    }

    pub fn start_work(&mut self) -> Result<(), String> {
        match self._worker_handle {
            None => {
                println!("At start of work function");
                let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
                self._sender = Some(tx.clone());
                let mut pin = self._out_pin.take();
                self._worker_handle = Some(thread::spawn( move || {
                    loop{
                        println!("Waiting for input");
                        match rx.try_recv(){
                            Ok(val) => 
                            {
                                if val == 1 {
                                    println!("Thread ending");
                                    return;
                                }        
                                println!("Do something");
                            },
                            _ => (),
                        }
                        match pin {
                            Some(ref mut pin_val) =>{
                                pin_val.set_high();
                                thread::sleep(Duration::from_millis(1000));
                                pin_val.set_low();
                                thread::sleep(Duration::from_millis(1000));
                            },
                            None => {
                                println!("Failed to get a pin");
                                thread::sleep(Duration::from_millis(1000));
                            }
                        }
                        
                    }
                }));
                Ok(())
            },
            _ => Err("Thread already running".to_string()),
        }
    }

    pub fn end_work(&mut self) -> Result<(), String>{

        match self._sender.as_mut() {
            Some(sender) => {
                println!("Signaling the end of the thread");
                sender.send(1).unwrap();
                self._worker_handle.take().map(JoinHandle::join);
                self._sender = None;
                self._worker_handle = None;
                Ok(())
            },
            None => Err("Could not end the thread".to_string())
        }
    }
}