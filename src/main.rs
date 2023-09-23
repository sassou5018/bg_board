use eframe::egui;
use inputbot::KeybdKey;
use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::{self, yield_now, JoinHandle};
use stop_thread;
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    //run_bg_board();
    eframe::run_native(
        "App name title thingy",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

enum ass{
    stop(JoinHandle<()>),
    keepgoing(())
}

struct MyApp {
    start: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { start: false }
    }
}

const TRUE_TEXT: &str = "True";
const FALSE_TEXT: &str = "False";
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Egui Application");
            let button_text: &str;
            if self.start {
                ui.label("True");
                button_text = TRUE_TEXT;
            } else {
                ui.label("False");
                button_text = FALSE_TEXT;
            }
            if ui.button(button_text).clicked() {
                self.start = !self.start;
                //let (tx, rx) = mpsc::channel();
                let mut join_handle: Option<JoinHandle<()>> = None;


                if self.start{join_handle = Some(thread::spawn(|| {
                    run_bg_board();
                }))}
                
                //let _ = tx.send(ass::keepgoing(()));
                
                if !self.start{
                    println!("Stopping...");
                    unsafe{stop_thread::kill_thread_forcibly_exit_code(join_handle.expect("Errr getting join handle"), 0);};
                    join_handle = None;
                }                
                
            }
        });
    }
}

fn run_bg_board() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("audiofile.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    let source_buffered = source.buffered();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sink = Arc::new(Mutex::new(sink));
    KeybdKey::bind_all(move |e| {
        match e {
            //KeybdKey::QKey => stop_thread::kill_thread_forcibly_exit_code(),
            _ => {
                println!("character pressed: {:?}", e);
                sink.lock().unwrap().append(source_buffered.clone());
            }
        };
    });
    // match rx.try_recv() {
    //     Ok(a) => {
    //         match a{
    //             ass::keepgoing(_)=>{},
    //             ass::stop(b)=>{
    //                 println!("Terminating.");
    //                 unsafe {
    //                     stop_thread::kill_thread_forcibly_exit_code(b, 0);
    //                 };
    //             }
    //         }
            
    //     }
    //     Err(_) => {
    //         panic!("Some multiThreading thing broke");
    //     }
    // };
    inputbot::handle_input_events();
}
