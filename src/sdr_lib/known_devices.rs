use super::SdrDongle;

pub static KNOWN_DEVICES: &'static [SdrDongle] = &[
    SdrDongle {
        vendor_id: 0x0bda,
        product_id: 0x2832,
        name: "Generic RTL2832U",
    },
    SdrDongle {
        vendor_id: 0x0bda,
        product_id: 0x2838,
        name: "Generic RTL2832U OEM",
    },
    SdrDongle {
        vendor_id: 0x0413,
        product_id: 0x6680,
        name: "DigitalNow Quad DVB-T PCI-E card",
    },
    SdrDongle {
        vendor_id: 0x0413,
        product_id: 0x6f0f,
        name: "Leadtek WinFast DTV Dongle mini D",
    },
    SdrDongle {
        vendor_id: 0x0458,
        product_id: 0x707f,
        name: "Genius TVGo DVB-T03 USB dongle (Ver. B)",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00a9,
        name: "Terratec Cinergy T Stick Black (rev 1)",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00b3,
        name: "Terratec NOXON DAB/DAB+ USB dongle (rev 1)",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00b4,
        name: "Terratec Deutschlandradio DAB Stick",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00b5,
        name: "Terratec NOXON DAB Stick - Radio Energy",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00b7,
        name: "Terratec Media Broadcast DAB Stick",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00b8,
        name: "Terratec BR DAB Stick",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00b9,
        name: "Terratec WDR DAB Stick",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00c0,
        name: "Terratec MuellerVerlag DAB Stick",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00c6,
        name: "Terratec Fraunhofer DAB Stick",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00d3,
        name: "Terratec Cinergy T Stick RC (Rev.3)",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00d7,
        name: "Terratec T Stick PLUS",
    },
    SdrDongle {
        vendor_id: 0x0ccd,
        product_id: 0x00e0,
        name: "Terratec NOXON DAB/DAB+ USB dongle (rev 2)",
    },
    SdrDongle {
        vendor_id: 0x1554,
        product_id: 0x5020,
        name: "PixelView PV-DT235U(RN)",
    },
    SdrDongle {
        vendor_id: 0x15f4,
        product_id: 0x0131,
        name: "Astrometa DVB-T/DVB-T2",
    },
    SdrDongle {
        vendor_id: 0x15f4,
        product_id: 0x0133,
        name: "HanfTek DAB+FM+DVB-T",
    },
    SdrDongle {
        vendor_id: 0x185b,
        product_id: 0x0620,
        name: "Compro vendor_ideomate U620F",
    },
    SdrDongle {
        vendor_id: 0x185b,
        product_id: 0x0650,
        name: "Compro vendor_ideomate U650F",
    },
    SdrDongle {
        vendor_id: 0x185b,
        product_id: 0x0680,
        name: "Compro vendor_ideomate U680F",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd393,
        name: "GIGABYTE GT-U7300",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd394,
        name: "DIKOM USB-DVBT HD",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd395,
        name: "Peak 102569AGPK",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd397,
        name: "KWorld KW-UB450-T USB DVB-T Pico TV",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd398,
        name: "Zaapa ZT-MINDVBZP",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd39d,
        name: "SVEON STV20 DVB-T USB & FM",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd3a4,
        name: "Twintech UT-40",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd3a8,
        name: "ASUS U3100MINI_PLUS_V2",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd3af,
        name: "SVEON STV27 DVB-T USB & FM",
    },
    SdrDongle {
        vendor_id: 0x1b80,
        product_id: 0xd3b0,
        name: "SVEON STV21 DVB-T USB & FM",
    },
    SdrDongle {
        vendor_id: 0x1d19,
        product_id: 0x1101,
        name: "Dexatek DK DVB-T Dongle (Logilink VG0002A)",
    },
    SdrDongle {
        vendor_id: 0x1d19,
        product_id: 0x1102,
        name: "Dexatek DK DVB-T Dongle (MSI DigiVox mini II V3.0)",
    },
    SdrDongle {
        vendor_id: 0x1d19,
        product_id: 0x1103,
        name: "Dexatek Technology Ltd. DK 5217 DVB-T Dongle",
    },
    SdrDongle {
        vendor_id: 0x1d19,
        product_id: 0x1104,
        name: "MSI DigiVox Micro HD",
    },
    SdrDongle {
        vendor_id: 0x1f4d,
        product_id: 0xa803,
        name: "Sweex DVB-T USB",
    },
    SdrDongle {
        vendor_id: 0x1f4d,
        product_id: 0xb803,
        name: "GTek T803",
    },
    SdrDongle {
        vendor_id: 0x1f4d,
        product_id: 0xc803,
        name: "Lifeview LV5TDeluxe",
    },
    SdrDongle {
        vendor_id: 0x1f4d,
        product_id: 0xd286,
        name: "MyGica TD312",
    },
    SdrDongle {
        vendor_id: 0x1f4d,
        product_id: 0xd803,
        name: "PROlectrix DV107669",
    },
];
