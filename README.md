# ch9328kbd
A tiny app to send direct key codes into CH9328-based serial-to-USB-keyboard controller

CH9328 (https://lcsc.com/product-detail/USB-ICs_WCH-Jiangsu-Qin-Heng-CH9328_C82452.html) is a pretty curious little chip: it emulates a USB keyboard on one side, and on the other side it receives the data from a regular serial port, and that data is used to control the emulated USB keyboard... Cool, huh ?

The controller has several modes, depending on whether the IO pins are left hanging or pulled to the ground during the boot...

If you pull IO2 to the ground, this unlocks a pretty cool mode whereby the chip simply "bridges" the 8-byte chunks of data received via the serial port as USB keyboard HID reports.

The format must be 8 bytes of keypress event, followed by 8 bytes of key release event, as per https://wiki.osdev.org/USB_Human_Interface_Devices#USB_keyboard

This small program, puts the terminal into raw mode, and converts (somewhat) the received keys into the corresponding USB HID report events.

Together with the video playback from an HDMI capture device like https://www.amazon.com/MavisLink-Capture-Definition-Acquisition-Broadcasting/dp/B087C3NG9W/ref=sr_1_3?dchild=1&keywords=hdmi+capture&qid=1599862416&sr=8-3 this gives a very simple ad-hoc KVM solution, or simply a replacement for an extra display and keyboard.

