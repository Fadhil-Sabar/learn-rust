fn main() {
    //* di rust, ada konsep match yang mirip if else tapi ini bisa berbagai kondisi
    //* kondisi ini unique di rust karena bisa match berbagai pattern, dan commonly used pake enum

    enum Negara {
        Indonesia,
        Malaysia,
        Jepang
    }

    struct Orang {
        asal_negara: Negara
    }

    let orang_random = Orang {
        asal_negara: Negara::Indonesia
    };

    //* ini adalah kondisi match, jadi mirip switch sebenernya di bahasa lain */
    match orang_random.asal_negara {
        //* sisi kiri ini kondisinya, sisi kanan apa yang dilakuin atau return */
        Negara::Indonesia => println!("Ada Indonesia coy!"),
        Negara::Jepang => println!("Konnichiwa"),
        Negara::Malaysia => println!("Betul, betul, betul"),
        // * ada juga other => println!("Jir lu orang mana"), ini untuk handle yang lain
        // * atau pake _ juga bisa, rust tau kalo ini ga bakal dipake jadi gaakan ada warning
    }

    //* ada lagi namanya if let dan else, ini sama kaya match */
    //* sama aja kaya match, tapi dibalik sedikit */
    //* sisi kiri untuk kondisinya, sisi kanan untuk variable yang mau dicek */
    if let Negara::Indonesia = orang_random.asal_negara {
        println!("Rendang")
    } else {
        println!("Anjay bule coy")
    };

    //* ada juga let else, ini sesuai namanya untuk catch kondisi yang tidak match */
    let Negara::Indonesia = orang_random.asal_negara else {
        println!("Orang luar ya coy");
        return ;
    };
}
