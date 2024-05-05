use image::Rgb;

pub struct MatchColors {
    pub match_colors_star: [Rgb<u8>; 5],
    pub match_colors_lock: [Rgb<u8>; 3],
    pub match_colors_discard: [Rgb<u8>; 3],
    pub match_colors_equipper: [(&'static str, Rgb<u8>); 43],
}

pub const MATCH_COLORS: MatchColors = MatchColors {
    match_colors_star: [
        Rgb([113, 119, 139]), // todo
        Rgb([42, 143, 114]),  // todo
        Rgb([96, 142, 197]),
        Rgb([157, 117, 206]),
        Rgb([193, 158, 112]),
    ],
    match_colors_lock: [
        Rgb([18, 18, 18]),      // locked
        Rgb([249, 249, 249]),   // unlocked
        Rgb([116, 108, 99]),    // discard
    ],
    match_colors_discard: [
        Rgb([235, 77, 61]),     // discard
        Rgb([249, 249, 249]),   // not discard
        Rgb([115, 108, 98]),    // locked
    ],
    match_colors_equipper: [
        ("Argenti", Rgb([216, 174, 161])),
        ("Arlan", Rgb([146, 134, 124])),
        ("Asta", Rgb([188, 130, 117])),
        ("Bailu", Rgb([160, 127, 174])),
        ("BlackSwan", Rgb([252, 242, 239])),
        ("Blade", Rgb([191, 162, 162])),
        ("Bronya", Rgb([83, 66, 83])),
        ("Clara", Rgb([181, 107, 129])),
        ("DanHeng", Rgb([124, 100, 100])),
        ("DanHengImbibitorLunae", Rgb([181, 169, 163])),
        ("DrRatio", Rgb([134, 120, 143])),
        ("FuXuan", Rgb([231, 166, 145])),
        ("Gepard", Rgb([192, 199, 223])),
        ("Guinaifen", Rgb([219, 137, 111])),
        ("Hanya", Rgb([247, 238, 232])),
        ("Herta", Rgb([246, 239, 227])),
        ("Himeko", Rgb([177, 92, 85])),
        ("Hook", Rgb([190, 161, 86])),
        ("Huohuo", Rgb([230, 250, 250])),
        ("Jingliu", Rgb([193, 194, 218])),
        ("JingYuan", Rgb([169, 154, 147])),
        ("Kafka", Rgb([126, 50, 80])),
        ("Luka", Rgb([218, 198, 183])),
        ("Luocha", Rgb([191, 160, 116])),
        ("Lynx", Rgb([247, 213, 197])),
        ("March7th", Rgb([251, 243, 243])),
        ("Misha", Rgb([234, 215, 213])),
        ("Natasha", Rgb([238, 208, 196])),
        ("Pela", Rgb([241, 217, 217])),
        ("Qingque", Rgb([18, 27, 11])),
        ("RuanMei", Rgb([129, 101, 101])),
        ("Sampo", Rgb([241, 217, 213])),
        ("Seele", Rgb([91, 65, 111])),
        ("Serval", Rgb([158, 141, 150])),
        ("SilverWolf", Rgb([222, 210, 210])),
        ("Sushang", Rgb([101, 65, 58])),
        ("Tingyun", Rgb([127, 116, 57])),
        ("TopazNumby", Rgb([254, 250, 246])),
        ("Trailblazer_Preservation", Rgb([153, 125, 111])),
        ("Welt", Rgb([158, 114, 99])),
        ("Xueyi", Rgb([250, 242, 230])),
        ("Yanqing", Rgb([255, 242, 232])),
        ("Yukong", Rgb([174, 167, 174]))
    ]
};
