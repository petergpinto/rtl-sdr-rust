//https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/tuner_r82xx.c

struct FreqencyRange {
    frequency: u32,
    open_d: u16,
    rf_mux_ploy: u16,
    tf_c: u16,
    xtal_cap20p: u16,
    xtal_cap10p: u16,
    xtal_cap0p: u16
}

const FREQUENCY_RANGES: &[FreqencyRange] = &[
    FreqencyRange {
        frequency: 0,
        open_d: 0x08,
        rf_mux_ploy: 0x02,
        tf_c: 0xdf,
        xtal_cap20p: 0x02,
        xtal_cap10p: 0x01,
        xtal_cap0p: 0x00,
    }
];