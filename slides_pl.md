---
title: Rust
sub_title: Szybki przegląd i wprowadzenie
author: Mikołaj Juda
---

Rust: Przegląd
===

Rust jest kompilowanym, wieloparadygmatowym językiem ogólnego przeznaczenia
skupiającym się, między innymi, na bezpieczeństwie, niezawodności i wydajności.

<!-- column_layout: [6, 2] -->

<!-- column: 1 -->

![](rust_logo.png)

<!-- column: 0 -->

## ważne cechy:
- bezpieczeństwo pamięci bez potrzeby automatycznego odśmiecania (borrow checker)
- zapobieganie wyścigom danych między wątkami (m.in. borrow checker)
- silny system typów inspirowany językami funkcyjnymi (bazuje na nim obsługa błędów)
- możliwość programowania niskopoziomowego (wskaźniki, unsafe, inline assembly)
- interoperacyjność z językiem C (i nie tylko) (Wywoływanie funkcji z innych języków nie jest bezpieczne)
- makra (deklaratywne i proceduralne)
- wysokopoziomowe funkcje (np. traits, uogólnienia, iteratory) (zwykle)
nie wiążą się z wyższym kosztem wykonania niż implementacja "ręczna" (zero-cost abstractions)
- wydajność porównywalna z językiem C
- użycie infrastruktury LLVM (wieloplatformowość)
- z kompilatorem dostarczony jest menadżer pakietów Cargo zintegrowany z repozytorium [crates.io](https://crates.io) (i inne narzędzia)
- open-source

<!-- reset_layout -->
<!-- end_slide -->
