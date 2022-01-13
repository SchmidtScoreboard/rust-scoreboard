# Introduction

Interested in building a Schmidt Scoreboard by yourself? This document will list out all the parts you need, all the construction steps you have to take, and walk you through automatic software (flashing an SD card) and manually setting up a stock Raspbian image with drivers and files (coming soon). 

I would recommend reading through the ENTIRE guide before you purchase anything. Please feel free to reach out with questions, but know that I may be unable to resolve your issue for you. I provide no warranty on this guide.


# Parts

Below is a table showing all the parts you will need to build a Schmidt Scoreboard. I’ve included a column for prices as of **early March 2021**—pricing and availability are out of my control and subject to change.

| Part | Link | Price | Notes |
|:--- |:---- |:----| :----|
| Raspberry Pi 4B 2GB |https://www.adafruit.com/product/4292  | $35 | You can use a Pi3, or different variants of the Pi4. You will have to tweak a setting
| Adafruit RGB Matrix Bonnet | https://www.adafruit.com/product/3211 | $14.95 |
| MicroSD Card (8GB) | https://www.amazon.com/KEXIN-Micro-MicroSDHC-UHS-I-Memory/dp/B085ZVG84C/ref=sr_1_10?dchild=1&keywords=8gb+micro+sd+card&qid=1594683632&sr=8-10 | $4.00 | Any >4GB microSD card should be fine
| RGB Matrix | https://www.amazon.com/AZERONE-digital-display-2121SMD-256x128mm/dp/B07F2JW8D3/ref=sr_1_2?dchild=1&keywords=rgb+matrix&qid=1594683576&sr=8-2 | $31.99 | Make certain that your RGB matrix has the correct screw holes to line up with the 3D printed housing—if they are different, you will have to modify the 3D model. See the photos below—the back of the matrix I use should be distinctive
| Button | https://www.adafruit.com/product/471 | $5.95 | There are other color options for this arcade button that should fit the slot.
| Power Adapter | https://www.amazon.com/gp/product/B01K0608A0/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1 | $6.95 | Any 5V 4A power adapter should be fine
| Button Wires | https://www.adafruit.com/product/3835  |$4.95 | Optional—you can solder jumper cables from the button to the board, but I like that these are easy to remove
| IDC Cable | https://www.adafruit.com/product/4170 | $1.95 | The IDC cable that comes with the matrix is too short
| WiFi Dongle | https://www.amazon.com/gp/product/B008IFXQFU/ref=ppx_yo_dt_b_search_asin_title? | $9.99 | RPi’s built in WiFi radio is subpar and barely penetrates the case.
| 3D Printed Housing | Links to the files can be found [here]() | It costs me $15 at $0.10 per gram to get both parts printed at my local library—if you don’t have access to a 3D printer (or a big enough 3D printer), try your local library! I’ve had the best luck with black PLA+ filament
| Gorilla Glue | Hardware store or target | <$5 | Used to secure nuts into 3D print.
| M3 Screws and Nuts | Your local hardware store | <$5 | All screws for this are M3. You'll need 8 10mm screws and 4 8mm screws.
| Jumper cables | Electronics store, Amazon, anywhere. | <$5 | You don’t need super long jumper cables.

The total price of parts to build a Scoreboard is about $135, not including shipping from various sources. If you have your own 3D printer, you can save some money. The most variable priced item is the Matrix—it can be difficult to find in the right size, but have patience!


# Hardware

### Step 1: Prep the 3D Printed Housing

1. Begin with the Scoreboard housing. Start by clearing off all the excess filament, brim, and support structures. I’ve found needle nose pliers work to rip the edges off, and sandpaper helps sand down any rough edges.
2. On the Scoreboard housing, squeeze Gorilla Glue into the 4 nut holes that will hold the Raspberry Pi. Then, using pliers, push an M3 nut into each hole as shown in the photo. The nuts should be roughly flush with the surface of the Scoreboard.
3. Repeat this step on the curved hood where the hood will attach to the Scoreboard.
4. Let the glue dry overnight.

### Step 2: Prep the Matrix Bonnet

1. You’ll need to solder at 4 points on the Matrix Bonnet.
2. Solder a jumper from pin 4 -> pin 18. This is the hardware mod described [here](https://github.com/hzeller/rpi-rgb-led-matrix#improving-flicker) that reduces flickering
3. Solder a jumper from the 3.3V pin
4. Solder a jumper from the ground pin

### Step 3: Software Setup

1. You can download the full Scoreboard software image that is flash-able onto an 8GB SD card [here]()
2. If you’d prefer to set things up yourself, you’ll have to be patient—this code is in flux, and there does not exist an install script *yet*. One day!
3. I use [Etcher](https://www.balena.io/etcher/) to write the image to the SD card.

### Step 4: Assemble!

1. Using M3 screws, screw the Scoreboard housing into the RGB matrix. Be sure not to over tighten or use long screws, as you could damage the RGB Matrix. Make sure that the arrows are facing up! ⬆️
2. Plug the WiFi dongle into the Raspberry Pi.
3. Insert the SD card into the Raspberry Pi.
2. Using M3 screws, secure the Raspberry Pi to the housing. These screws may strip the holes of the Raspberry Pi a small amount.
3. Plug the Matrix Bonnet into the Raspberry Pi so that the    DC barrel plug lines up with the rectangular hole on the side of the housing.
4. Using the screw terminals on the Matrix Bonnet, connect the power cable from the bonnet to the RGB Matrix.
5. Plug the longer IDC cable into the Matrix Bonnet and the Scoreboard itself.
6. Slide the button into the hole on the right side of the Scoreboard housing, then connect the button wire to the jumper cables (or just solder the button directly to the matrix)
6. Plug in the power adapter and power on!

### Step 5: Systems Check!

1. We should make sure you scoreboard is fully operational. By default, you should be able to use the Scoreboard app to set up and connect to WiFi, but you’ll need an API key in order to fetch game data.
2. When you reach this point, send me an email at [mark.schmidt@hey.com](mark.schmidt@hey.com) and I can get you connected. Or, you can fork [this repo](https://github.com/SchmidtScoreboard/rust-scoreboard) and set up your own AWS service.
3. Copy the API key into ```/var/lib/scoreboard/secrets.txt``` by SSHing into the scoreboard, mounting the SD card, or connecting to a mouse/keyboard/monitor. 
4. Check that all sports work.

### Step 6: Seal it up!

1. Using M3 screws, lock the Scoreboard hood onto the base enclosure. Your scoreboard is now done!


