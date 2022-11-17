//use std::future::join;
//use rust_gpiozero::*;
use std::thread;
//use std::time::Duration;
//use std::process::exit;
use rand::Rng;

//use gpio::{GpioIn, GpioOut};
use rust_gpiozero::LED;

//this struct have values of power on and off and color of leds based on current status
struct Message
{
    power : u64,
    led_status: u64,
}
//Made it static bacause it can be accesible in main function also.

static mut MSG: Message = Message{power:1, led_status:0 };

fn main()
{
    //Make thread that will check and send status for lEDS and auths
    //This will continuously wait for the information about blinking and authentications

     thread::spawn(move|| unsafe {

        loop {

            //Currently generating randoms vales for states of LED and power but later on shift this work with original message that will come from server.

            let mut rng1 = rand::thread_rng();
            let mut rng2 = rand::thread_rng();

            let no_of_led :u64 = rng1.gen_range(0..3); //this will tell the status of lEDs
            let no_of_pwr :u64 = rng2.gen_range(0..2); //this will tell us tha power should be on or off

            println!("{}",no_of_led);

            MSG.led_status = no_of_led;
            MSG.power = no_of_pwr;

            //Now machine should be shout down.

            if no_of_pwr == 0
            {
                return;
            }
        }
    });

    //GPIO pins for LEDs
    // Create a new LED attached to Pin 17,27,22

    let red_led = LED::new(17);
    let green_led = LED::new(27);
    let orange_led = LED::new(22);

    unsafe {
        loop {
            let mut status = MSG.led_status; //Status = 0 -> Red Led, Status = 1 -> Green Led , Status = 2 -> Orange Led
            let mut power = MSG.power;

            if power == 1
            {
                if status == 0
                {
                    //means red led
                    green_led.off();
                    orange_led.off();
                    red_led.on();
                }

                if status == 1
                {
                    //means green led
                    red_led.off();
                    orange_led.off();
                    green_led.on();
                }

                if status == 2 {
                    //means orange led
                    green_led.off();
                    red_led.off();
                    orange_led.on();
                }
            }
            else if power == 0
            {
                //Before shutdown all LEDS should be off
                green_led.off();
                red_led.off();
                orange_led.off();
                println!("ShutDown! ");
                return;
            }
        }
    }
}
