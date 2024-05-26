/* https://osmocom.org/projects/rtl-sdr/repository/rtl-sdr/revisions/master/entry/src/librtlsdr.c#L84
 * FIR coefficients.
 *
 * The filter is running at XTal frequency. It is symmetric filter with 32
 * coefficients. Only first 16 coefficients are specified, the other 16
 * use the same values but in reversed order. The first coefficient in
 * the array is the outer one, the last, the last is the inner one.
 * First 8 coefficients are 8 bit signed integers, the next 8 coefficients
 * are 12 bit signed integers. All coefficients have the same weight.
 *
 * Default FIR coefficients used for DAB/FM by the Windows driver,
 * the DVB driver uses different ones
 */
static FIR_DEFAULT: &[i32] = &[
    -54, -36, -41, -40, -32, -14, 14, 53, /* 8 bit signed */
    101, 156, 215, 273, 327, 372, 404, 421, /* 12 bit signed */
];
