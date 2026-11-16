#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
fn string_concat(parts: &[&str]) -> Rc<str> {
    let mut joined = Rc::<[u8]>::new_uninit_slice(parts.iter().map(|part| part.len()).sum());
    let bytes = Rc::get_mut(&mut joined).unwrap().as_mut_ptr() as *mut u8;
    let mut at = 0;
    for part in parts {
        unsafe { std::ptr::copy_nonoverlapping(part.as_ptr(), bytes.add(at), part.len()) };
        at += part.len();
    }
    unsafe { Rc::from_raw(Rc::into_raw(joined.assume_init()) as *const str) }
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Markup(Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US0, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<UH2>, Rc<UH3>),
}
#[derive(Clone)]
enum US2 {
    US2_GroupUnclosed,
    US2_ClassUnclosed,
    US2_CloseWithoutOpen,
    US2_RepeatOfNothing,
    US2_RepeatStacked,
    US2_CountInvalid,
    US2_LookaroundUnsupported,
    US2_BackreferenceUnsupported,
    US2_WordBoundaryUnsupported,
    US2_AnchorInside,
    US2_UnicodeTableMissing,
    US2_CaseFoldBeyondAscii,
    US2_EscapeUnknown,
    US2_FlagUnknown,
    US2_HexInvalid,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum UH5 {
    UH5_Nil,
    UH5_Cons(u32, u32, Rc<UH5>),
}
#[derive(Clone)]
enum US4 {
    US4_Normal(Rc<UH5>),
}
#[derive(Clone)]
enum US5 {
    US5_Bounded(u32),
    US5_Unbounded,
}
#[derive(Clone)]
enum UH4 {
    UH4_Nothing,
    UH4_Epsilon,
    UH4_Set(US4),
    UH4_Seq(Rc<UH4>, Rc<UH4>),
    UH4_Alt(Rc<UH4>, Rc<UH4>),
    UH4_And(Rc<UH4>, Rc<UH4>),
    UH4_Not(Rc<UH4>),
    UH4_Star(Rc<UH4>),
    UH4_Repeat(Rc<UH4>, u32, US5),
}
#[derive(Clone)]
enum UH6 {
    UH6_Nil,
    UH6_Cons(u32, Rc<UH6>),
}
#[derive(Clone)]
enum US3 {
    US3_Got(Rc<UH4>, Rc<UH6>, bool, bool, bool, bool, bool),
    US3_Stopped(US2),
}
#[derive(Clone)]
enum US6 {
    US6_RustParsed(Rc<UH4>),
    US6_RustRefused(US2),
}
#[derive(Clone)]
enum UH7 {
    UH7_Nil,
    UH7_Cons(i32, Rc<UH7>),
}
#[derive(Clone)]
enum UH8 {
    UH8_Nil,
    UH8_Cons(Rc<UH4>, Rc<UH8>),
}
#[derive(Clone)]
enum US7 {
    US7_Before,
    US7_Same,
    US7_After,
}
#[derive(Clone)]
enum US8 {
    US8_Unpinned,
    US8_PinnedText,
    US8_PinnedLine,
    US8_WordEdge,
}
#[derive(Clone)]
enum UH10 {
    UH10_NothingRejects,
    UH10_SetRejects,
    UH10_SeqLeftRejects(Rc<UH10>),
    UH10_SeqRightRejects(Rc<UH10>),
    UH10_AltRejects(Rc<UH10>, Rc<UH10>),
    UH10_AndLeftRejects(Rc<UH10>),
    UH10_AndRightRejects(Rc<UH10>),
    UH10_NotOfAccepting(Rc<UH9>),
    UH10_RepeatRejects(Rc<UH10>),
}
#[derive(Clone)]
enum UH9 {
    UH9_EpsilonAccepts,
    UH9_StarAccepts,
    UH9_RepeatFromZero,
    UH9_RepeatOfNullable(Rc<UH9>),
    UH9_SeqAccepts(Rc<UH9>, Rc<UH9>),
    UH9_AltLeftAccepts(Rc<UH9>),
    UH9_AltRightAccepts(Rc<UH9>),
    UH9_AndAccepts(Rc<UH9>, Rc<UH9>),
    UH9_NotOfRejecting(Rc<UH10>),
}
#[derive(Clone)]
enum US9 {
    US9_Accepts(Rc<UH9>),
    US9_Rejects(Rc<UH10>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US10 {
    US10_StartAnchor(US8, Rc<UH6>),
    US10_EndAnchor(US8, Rc<UH6>),
    US10_WordAnchor(Rc<UH6>),
    US10_NoAnchor,
}
#[derive(Clone)]
enum US11 {
    US11_FlagsSet(bool, bool, bool, bool, bool, Rc<UH6>),
    US11_FlagsScoped(bool, bool, bool, bool, bool, Rc<UH6>),
    US11_FlagsBad,
    US11_NotFlags,
}
#[derive(Clone)]
enum US12 {
    US12_Some(bool, bool, bool, bool, bool),
    US12_None,
}
#[derive(Clone)]
enum US13 {
    US13_CountNumber(u32, Rc<UH6>),
    US13_NoCountNumber,
}
#[derive(Clone)]
enum US14 {
    US14_Some(Rc<UH6>),
    US14_None,
}
#[derive(Clone)]
enum US15 {
    US15_ClassGot(US4, Rc<UH6>),
    US15_ClassStopped(US2),
}
#[derive(Clone)]
enum US16 {
    US16_EscChar(u32, Rc<UH6>),
    US16_EscSet(US4, Rc<UH6>),
    US16_EscStopped(US2),
}
#[derive(Clone)]
enum US17 {
    US17_Some(US15),
    US17_None,
}
#[derive(Clone)]
enum US18 {
    US18_Some(Rc<UH6>, Rc<UH6>),
    US18_None,
}
#[derive(Clone)]
enum UH11 {
    UH11_Nil,
    UH11_Cons(Rc<str>, Rc<UH5>, Rc<UH11>),
}
#[derive(Clone)]
enum US19 {
    US19_Some(US4),
    US19_None,
}
#[derive(Clone)]
enum US20 {
    US20_Some(u32, Rc<UH6>),
    US20_None,
}
#[derive(Clone)]
enum US21 {
    US21_Some(Rc<str>, Rc<str>),
    US21_None,
}
fn agreement_patterns_0() -> Rc<UH0> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abc"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^ab|cd$"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?i)k"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^[a-z&&[^aeiou]]+$"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?m)^b$"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a{2,3}"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\d+"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\w+\\s\\w"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?s)a.c"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.c"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[[:alpha:]]+[0-9]"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?x) a b # note\n c"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x*y?z+"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^(a|bc)*d$"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[^a-c]x"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x41|\\x{3b1}"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?-u)\\w+$"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\W"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?i)[k-m]{2}"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\D\\s\\D"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\p{Greek}+"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\pL\\PL"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[\\p{Lu}\\d]"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\P{N}\\p{gc=Nd}"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bk\\b"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\b\\d"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a\\b"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("k.\\b\\d"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\w\\b\\W"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v30: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28, v29));
    let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27, v30));
    let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26, v31));
    let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v25, v32));
    let v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24, v33));
    let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v34));
    let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v35));
    let v37: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v36));
    let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v37));
    let v39: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v38));
    let v40: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v39));
    let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v40));
    let v42: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v41));
    let v43: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v42));
    let v44: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v43));
    let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v44));
    let v46: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v45));
    let v47: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v46));
    let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v47));
    let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v48));
    let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v49));
    let v51: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v50));
    let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v51));
    let v53: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v52));
    let v54: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v53));
    let v55: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v54));
    let v56: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v55));
    let v57: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v56));
    Rc::new(UH0::UH0_Cons(v0, v57))
}
fn refusal_examples_1() -> Rc<UH0> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?=a)"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(a)\\1"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a*\\bb"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a(^b)"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\p{Klingon}"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?i)\\p{Lu}"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?i)\\x{e9}"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a**"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v9));
    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v10));
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v11));
    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v12));
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v13));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v14));
    Rc::new(UH0::UH0_Cons(v0, v15))
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method2(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn first_n_3(mut v0: i32, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let mut v5: bool = v0 <= 0i32;
                if v5 {
                    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    return method4(v2, v6);
                } else {
                    let v8: i32 = v0.wrapping_sub(1i32);
                    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v2));
                    (v0, v1, v2) = (v8, v4, v9);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method4(v2, v12);
            }
        }
    }
}
fn run_of_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: bool = v2 <= v1;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let mut v5: i32 = v2.wrapping_sub(1i32);
        let v6: Rc<str> = string_slice(&v0, v1 as i64, v5 as i64);
        v6
    }
}
fn entity_9(mut v0: u8) -> Rc<str> {
    let mut v1: bool = v0 == b'&';
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&amp;"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: bool = v0 == b'<';
        if v3 {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&lt;"); } LIT.with(|lit| lit.clone()) };
            v4
        } else {
            let mut v5: bool = v0 == b'>';
            if v5 {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&gt;"); } LIT.with(|lit| lit.clone()) };
                v6
            } else {
                let v7: bool = v0 == b'"';
                if v7 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&quot;"); } LIT.with(|lit| lit.clone()) };
                    v8
                } else {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v9
                }
            }
        }
    }
}
fn escape_step_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let v4: u8 = v0.clone().as_bytes()[v1 as usize];
    let mut v5: Rc<str> = entity_9(v4);
    let v6: bool = &*v5 == "";
    if v6 {
        (v2, v3)
    } else {
        let v7: i32 = v1.wrapping_add(1i32);
        let v8: Rc<str> = run_of_7(v0, v2, v1);
        let v10: Rc<str> = string_concat(&[&*v3, &*v8, &*v5]);
        (v7, v10)
    }
}
fn escape_from_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let mut v5: bool = v1 >= v4;
        if v5 {
            let v6: Rc<str> = run_of_7(v0, v2, v1);
            let v7: Rc<str> = string_concat(&[&*v3, &*v6]);
            return v7;
        } else {
            let (mut v8, mut v9): (i32, Rc<str>) = escape_step_8(v0.clone(), v1, v2, v3);
            let v10: i32 = v1.wrapping_add(1i32);
            (v1, v2, v3) = (v10, v8, v9);
            continue;
        }
    }
}
fn escape_5(mut v0: Rc<str>) -> US0 {
    let mut v1: i32 = 0i32;
    let v2: i32 = 0i32;
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = escape_from_6(v0, v1, v2, v3);
    US0::US0_Markup(v4)
}
fn markup_text_13(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Markup(v1) => {
            let mut v1: Rc<str> = v1;
            v1
        }
    }
}
fn attribute_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v4: US0 = escape_5(v0);
    let mut v5: Rc<str> = markup_text_13(v4);
    let v7: Rc<str> = string_concat(&[" ", &*v1, "=\"", &*v5, "\""]);
    v7
}
fn attributes_11(mut v0: Rc<UH1>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<str> = attribute_12(v2, v1);
            let mut v5: Rc<str> = attributes_11(v3);
            let v6: Rc<str> = string_concat(&[&*v4, &*v5]);
            v6
        }
        UH1::UH1_Nil => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
    }
}
fn element_10(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: US0) -> US0 {
    let v4: Rc<str> = attributes_11(v1);
    let v7: Rc<str> = markup_text_13(v2);
    let v11: Rc<str> = string_concat(&["<", &*v0, &*v4, ">", &*v7, "</", &*v0, ">"]);
    US0::US0_Markup(v11)
}
fn heads_20(mut v0: u32, mut v1: Rc<UH6>) -> bool {
    match Rc::unwrap_or_clone(v1) {
        UH6::UH6_Cons(v2, _) => {
            let v2: u32 = v2;
            let v4: bool = v2 == v0;
            v4
        }
        UH6::UH6_Nil => {
            false
        }
    }
}
fn rust_parsed_of_19(mut v0: US3) -> US6 {
    match v0 {
        US3::US3_Got(v1, v2, _, _, _, _, _) => {
            let mut v1: Rc<UH4> = v1;
            let v2: Rc<UH6> = v2;
            let v8: u32 = 41u32;
            let mut v9: bool = heads_20(v8, v2);
            if v9 {
                let v10: US2 = US2::US2_CloseWithoutOpen;
                US6::US6_RustRefused(v10)
            } else {
                US6::US6_RustParsed(v1)
            }
        }
        US3::US3_Stopped(v14) => {
            let v14: US2 = v14;
            US6::US6_RustRefused(v14)
        }
    }
}
fn codes_to_u32_22(mut v0: Rc<UH7>) -> Rc<UH6> {
    match Rc::unwrap_or_clone(v0) {
        UH7::UH7_Cons(v1, v2) => {
            let mut v1: i32 = v1;
            let v2: Rc<UH7> = v2;
            let v3: u32 = (v1 as u32);
            let v4: Rc<UH6> = codes_to_u32_22(v2);
            Rc::new(UH6::UH6_Cons(v3, v4))
        }
        UH7::UH7_Nil => {
            { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method23(mut v0: Rc<Vec<char>>, mut v1: i32, mut v2: i32, mut v3: Rc<UH7>) -> Rc<UH7> {
    loop {
        let v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let mut v5: usize = ((v2) as usize);
            let v6: char = (v0)[v5].clone();
            let v7: i32 = v2.wrapping_add(1i32);
            let v8: u32 = u32::from(v6);
            let mut v9: i32 = (v8 as i32);
            let v10: Rc<UH7> = Rc::new(UH7::UH7_Cons(v9, v3));
            (v2, v3) = (v7, v10);
            continue;
        }
    }
}
fn rust_codes_21(mut v0: Rc<str>) -> Rc<UH6> {
    let mut v1: Rc<Vec<char>> = Rc::new((&*v0).chars().collect());
    let v2: Rc<Vec<char>> = Rc::new((v1).as_ref().iter().rev().cloned().collect::<Vec<_>>());
    let v3: usize = (v2).len();
    let v4: i32 = ((v3) as i32);
    let mut v5: i32 = 0i32;
    let v6: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH7> = method23(v2, v4, v5, v6);
    codes_to_u32_22(v7)
}
fn default_flags_24() -> (bool, bool, bool, bool, bool) {
    (false, false, false, true, false)
}
fn alt_of_29(mut v0: Rc<UH8>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v0) {
        UH8::UH8_Cons(v2, v3) => {
            let v2: Rc<UH4> = v2;
            let v3: Rc<UH8> = v3;
            match &*v3 {
                UH8::UH8_Nil => {
                    v2
                }
                _ => {
                    let v4: Rc<UH4> = alt_of_29(v3);
                    Rc::new(UH4::UH4_Alt(v2, v4))
                }
            }
        }
        UH8::UH8_Nil => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
        }
    }
}
fn after_tie_37(mut v0: US7, mut v1: US7) -> US7 {
    match &v0 {
        US7::US7_Same => {
            v1
        }
        _ => {
            v0
        }
    }
}
fn order_empty_38(mut v0: Rc<UH5>) -> US7 {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Nil => {
            US7::US7_Same
        }
        _ => {
            US7::US7_Before
        }
    }
}
fn order_range_39(mut v0: u32, mut v1: u32, mut v2: u32, mut v3: u32) -> US7 {
    let v4: bool = v0 < v2;
    let v10: US7 = if v4 {
        US7::US7_Before
    } else {
        let v6: bool = v0 > v2;
        if v6 {
            US7::US7_After
        } else {
            US7::US7_Same
        }
    };
    match &v10 {
        US7::US7_Same => {
            let v11: bool = v1 < v3;
            if v11 {
                US7::US7_Before
            } else {
                let v13: bool = v1 > v3;
                if v13 {
                    US7::US7_After
                } else {
                    US7::US7_Same
                }
            }
        }
        _ => {
            v10
        }
    }
}
fn order_ranges_36(mut v0: Rc<UH5>, mut v1: Rc<UH5>) -> US7 {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v3, v4, v5) => {
            let v3: u32 = v3;
            let v4: u32 = v4;
            let mut v5: Rc<UH5> = v5;
            match Rc::unwrap_or_clone(v1) {
                UH5::UH5_Cons(v7, v8, v9) => {
                    let v7: u32 = v7;
                    let v8: u32 = v8;
                    let mut v9: Rc<UH5> = v9;
                    let v10: US7 = order_range_39(v3, v4, v7, v8);
                    let v11: US7 = order_ranges_36(v5, v9);
                    after_tie_37(v10, v11)
                }
                UH5::UH5_Nil => {
                    US7::US7_After
                }
            }
        }
        UH5::UH5_Nil => {
            order_empty_38(v1)
        }
    }
}
fn order_sets_35(mut v0: US4, mut v1: US4) -> US7 {
    let v3: Rc<UH5> = match v0 {
        US4::US4_Normal(v2) => {
            let v2: Rc<UH5> = v2;
            v2
        }
    };
    let mut v5: Rc<UH5> = match v1 {
        US4::US4_Normal(v4) => {
            let v4: Rc<UH5> = v4;
            v4
        }
    };
    order_ranges_36(v3, v5)
}
enum Group32 {
    M32(Rc<UH4>, Rc<UH4>),
    M33(Rc<UH4>, Rc<UH4>),
    M34(Rc<UH4>, Rc<UH4>, Rc<UH4>, Rc<UH4>),
    M40(Rc<UH4>, Rc<UH4>),
    M41(Rc<UH4>, Rc<UH4>),
    M42(Rc<UH4>, u32, US5, Rc<UH4>),
    M43(Rc<UH4>, u32, US5, Rc<UH4>, u32, US5),
}
fn group_32(mut st: Group32) -> US7 {
    loop {
        match st {
            Group32::M32(mut v0, mut v1) => {
                let v22: u32 = match &*v0 {
                    UH4::UH4_Alt(_, _) => {
                        4u32
                    }
                    UH4::UH4_And(_, _) => {
                        5u32
                    }
                    UH4::UH4_Epsilon => {
                        1u32
                    }
                    UH4::UH4_Not(_) => {
                        6u32
                    }
                    UH4::UH4_Nothing => {
                        0u32
                    }
                    UH4::UH4_Repeat(_, _, _) => {
                        8u32
                    }
                    UH4::UH4_Seq(_, _) => {
                        3u32
                    }
                    UH4::UH4_Set(_) => {
                        2u32
                    }
                    UH4::UH4_Star(_) => {
                        7u32
                    }
                };
                let v43: u32 = match &*v1 {
                    UH4::UH4_Alt(_, _) => {
                        4u32
                    }
                    UH4::UH4_And(_, _) => {
                        5u32
                    }
                    UH4::UH4_Epsilon => {
                        1u32
                    }
                    UH4::UH4_Not(_) => {
                        6u32
                    }
                    UH4::UH4_Nothing => {
                        0u32
                    }
                    UH4::UH4_Repeat(_, _, _) => {
                        8u32
                    }
                    UH4::UH4_Seq(_, _) => {
                        3u32
                    }
                    UH4::UH4_Set(_) => {
                        2u32
                    }
                    UH4::UH4_Star(_) => {
                        7u32
                    }
                };
                let v44: bool = v22 < v43;
                let v50: US7 = if v44 {
                    US7::US7_Before
                } else {
                    let v46: bool = v22 > v43;
                    if v46 {
                        US7::US7_After
                    } else {
                        US7::US7_Same
                    }
                };
                match &v50 {
                    US7::US7_Same => {
                        st = Group32::M33(v0, v1);
                        continue;
                    }
                    _ => {
                        return v50;
                    }
                }
            }
            Group32::M33(mut v0, mut v1) => {
                match &*v0 {
                    UH4::UH4_Seq(v7, v8) => {
                        let v7: Rc<UH4> = v7.clone();
                        let v8: Rc<UH4> = v8.clone();
                        match Rc::unwrap_or_clone(v1) {
                            UH4::UH4_Seq(v9, v10) => {
                                let mut v9: Rc<UH4> = v9;
                                let v10: Rc<UH4> = v10;
                                st = Group32::M34(v7, v8, v9, v10);
                                continue;
                            }
                            _ => {
                                return US7::US7_Same;
                            }
                        }
                    }
                    UH4::UH4_Set(v2) => {
                        let v2: US4 = v2.clone();
                        match Rc::unwrap_or_clone(v1) {
                            UH4::UH4_Set(v3) => {
                                let v3: US4 = v3;
                                return order_sets_35(v2, v3);
                            }
                            _ => {
                                return US7::US7_Same;
                            }
                        }
                    }
                    _ => {
                        st = Group32::M40(v0, v1);
                        continue;
                    }
                }
            }
            Group32::M34(mut v0, mut v1, mut v2, mut v3) => {
                let v4: US7 = order_re_32(v0, v2);
                match &v4 {
                    US7::US7_Same => {
                        st = Group32::M32(v1, v3);
                        continue;
                    }
                    _ => {
                        return v4;
                    }
                }
            }
            Group32::M40(mut v0, mut v1) => {
                match &*v0 {
                    UH4::UH4_Alt(v2, v3) => {
                        let v2: Rc<UH4> = v2.clone();
                        let v3: Rc<UH4> = v3.clone();
                        match Rc::unwrap_or_clone(v1) {
                            UH4::UH4_Alt(v4, v5) => {
                                let v4: Rc<UH4> = v4;
                                let mut v5: Rc<UH4> = v5;
                                st = Group32::M34(v2, v3, v4, v5);
                                continue;
                            }
                            _ => {
                                return US7::US7_Same;
                            }
                        }
                    }
                    UH4::UH4_And(v9, v10) => {
                        let mut v9: Rc<UH4> = v9.clone();
                        let v10: Rc<UH4> = v10.clone();
                        match Rc::unwrap_or_clone(v1) {
                            UH4::UH4_And(v11, v12) => {
                                let v11: Rc<UH4> = v11;
                                let v12: Rc<UH4> = v12;
                                st = Group32::M34(v9, v10, v11, v12);
                                continue;
                            }
                            _ => {
                                return US7::US7_Same;
                            }
                        }
                    }
                    _ => {
                        st = Group32::M41(v0, v1);
                        continue;
                    }
                }
            }
            Group32::M41(mut v0, mut v1) => {
                match Rc::unwrap_or_clone(v0) {
                    UH4::UH4_Not(v2) => {
                        let v2: Rc<UH4> = v2;
                        match Rc::unwrap_or_clone(v1) {
                            UH4::UH4_Not(v3) => {
                                let v3: Rc<UH4> = v3;
                                st = Group32::M32(v2, v3);
                                continue;
                            }
                            _ => {
                                return US7::US7_Same;
                            }
                        }
                    }
                    UH4::UH4_Repeat(v12, v13, v14) => {
                        let v12: Rc<UH4> = v12;
                        let v13: u32 = v13;
                        let v14: US5 = v14;
                        st = Group32::M42(v12, v13, v14, v1);
                        continue;
                    }
                    UH4::UH4_Star(v7) => {
                        let v7: Rc<UH4> = v7;
                        match Rc::unwrap_or_clone(v1) {
                            UH4::UH4_Star(v8) => {
                                let v8: Rc<UH4> = v8;
                                st = Group32::M32(v7, v8);
                                continue;
                            }
                            _ => {
                                return US7::US7_Same;
                            }
                        }
                    }
                    _ => {
                        return US7::US7_Same;
                    }
                }
            }
            Group32::M42(mut v0, mut v1, mut v2, mut v3) => {
                match Rc::unwrap_or_clone(v3) {
                    UH4::UH4_Repeat(v4, v5, v6) => {
                        let v4: Rc<UH4> = v4;
                        let mut v5: u32 = v5;
                        let v6: US5 = v6;
                        st = Group32::M43(v0, v1, v2, v4, v5, v6);
                        continue;
                    }
                    _ => {
                        return US7::US7_Same;
                    }
                }
            }
            Group32::M43(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5) => {
                let v6: bool = v1 < v4;
                let v12: US7 = if v6 {
                    US7::US7_Before
                } else {
                    let v8: bool = v1 > v4;
                    if v8 {
                        US7::US7_After
                    } else {
                        US7::US7_Same
                    }
                };
                match &v12 {
                    US7::US7_Same => {
                        let v31: US7 = match v2 {
                            US5::US5_Bounded(v13) => {
                                let v13: u32 = v13;
                                match v5 {
                                    US5::US5_Bounded(v14) => {
                                        let v14: u32 = v14;
                                        let v15: bool = v13 < v14;
                                        if v15 {
                                            US7::US7_Before
                                        } else {
                                            let v17: bool = v13 > v14;
                                            if v17 {
                                                US7::US7_After
                                            } else {
                                                US7::US7_Same
                                            }
                                        }
                                    }
                                    US5::US5_Unbounded => {
                                        US7::US7_Before
                                    }
                                }
                            }
                            US5::US5_Unbounded => {
                                match v5 {
                                    US5::US5_Bounded(_) => {
                                        US7::US7_After
                                    }
                                    US5::US5_Unbounded => {
                                        US7::US7_Same
                                    }
                                }
                            }
                        };
                        match &v31 {
                            US7::US7_Same => {
                                st = Group32::M32(v0, v3);
                                continue;
                            }
                            _ => {
                                return v31;
                            }
                        }
                    }
                    _ => {
                        return v12;
                    }
                }
            }
        }
    }
}
fn order_repeat_43(mut v0: Rc<UH4>, mut v1: u32, mut v2: US5, mut v3: Rc<UH4>, mut v4: u32, mut v5: US5) -> US7 {
    group_32(Group32::M43(v0, v1, v2, v3, v4, v5))
}
fn order_repeat_with_42(mut v0: Rc<UH4>, mut v1: u32, mut v2: US5, mut v3: Rc<UH4>) -> US7 {
    group_32(Group32::M42(v0, v1, v2, v3))
}
fn order_unary_41(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> US7 {
    group_32(Group32::M41(v0, v1))
}
fn order_pair_40(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> US7 {
    group_32(Group32::M40(v0, v1))
}
fn order_two_34(mut v0: Rc<UH4>, mut v1: Rc<UH4>, mut v2: Rc<UH4>, mut v3: Rc<UH4>) -> US7 {
    group_32(Group32::M34(v0, v1, v2, v3))
}
fn order_same_33(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> US7 {
    group_32(Group32::M33(v0, v1))
}
fn order_re_32(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> US7 {
    group_32(Group32::M32(v0, v1))
}
fn insert_leaf_31(mut v0: Rc<UH4>, mut v1: Rc<UH8>) -> Rc<UH8> {
    match &*v1 {
        UH8::UH8_Cons(v2, v3) => {
            let v2: Rc<UH4> = v2.clone();
            let v3: Rc<UH8> = v3.clone();
            let v4: US7 = order_re_32(v0.clone(), v2.clone());
            match v4 {
                US7::US7_After => {
                    let v6: Rc<UH8> = insert_leaf_31(v0, v3);
                    Rc::new(UH8::UH8_Cons(v2, v6))
                }
                US7::US7_Before => {
                    Rc::new(UH8::UH8_Cons(v0, v1))
                }
                US7::US7_Same => {
                    v1
                }
            }
        }
        UH8::UH8_Nil => {
            let v11: Rc<UH8> = { thread_local!{ static CASE: Rc<UH8> = Rc::new(UH8::UH8_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH8::UH8_Cons(v0, v11))
        }
    }
}
fn alt_leaves_30(mut v0: Rc<UH4>, mut v1: Rc<UH8>) -> Rc<UH8> {
    loop {
        match &*v0 {
            UH4::UH4_Alt(v2, v3) => {
                let v2: Rc<UH4> = v2.clone();
                let v3: Rc<UH4> = v3.clone();
                let v4: Rc<UH8> = alt_leaves_30(v3, v1);
                (v0, v1) = (v2, v4);
                continue;
            }
            UH4::UH4_Nothing => {
                return v1;
            }
            _ => {
                return insert_leaf_31(v0, v1);
            }
        }
    }
}
fn alt_28(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    let v4: bool = match &*v0 {
        UH4::UH4_Not(v2) => {
            let v2: Rc<UH4> = v2.clone();
            match Rc::unwrap_or_clone(v2) {
                UH4::UH4_Nothing => {
                    true
                }
                _ => {
                    false
                }
            }
        }
        _ => {
            false
        }
    };
    let v8: bool = if v4 {
        true
    } else {
        match &*v1 {
            UH4::UH4_Not(v5) => {
                let mut v5: Rc<UH4> = v5.clone();
                match Rc::unwrap_or_clone(v5) {
                    UH4::UH4_Nothing => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            _ => {
                false
            }
        }
    };
    if v8 {
        let mut v9: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) };
        Rc::new(UH4::UH4_Not(v9))
    } else {
        let v11: Rc<UH8> = { thread_local!{ static CASE: Rc<UH8> = Rc::new(UH8::UH8_Nil); } CASE.with(|case| case.clone()) };
        let v12: Rc<UH8> = alt_leaves_30(v1, v11);
        let v13: Rc<UH8> = alt_leaves_30(v0, v12);
        alt_of_29(v13)
    }
}
fn rust_join_27(mut v0: Rc<UH4>, mut v1: US3) -> US3 {
    match &v1 {
        US3::US3_Got(v2, v3, v4, v5, v6, v7, v8) => {
            let v2: Rc<UH4> = v2.clone();
            let v3: Rc<UH6> = v3.clone();
            let v4: bool = *v4;
            let mut v5: bool = *v5;
            let v6: bool = *v6;
            let v7: bool = *v7;
            let v8: bool = *v8;
            let mut v9: Rc<UH4> = alt_28(v0, v2);
            US3::US3_Got(v9, v3, v4, v5, v6, v7, v8)
        }
        _ => {
            v1
        }
    }
}
fn after_one_44(mut v0: Rc<UH6>) -> Rc<UH6> {
    match Rc::unwrap_or_clone(v0) {
        UH6::UH6_Cons(_, v2) => {
            let v2: Rc<UH6> = v2;
            v2
        }
        UH6::UH6_Nil => {
            { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn rust_after_branch_26(mut v0: bool, mut v1: US3) -> US3 {
    match &v1 {
        US3::US3_Got(v2, v3, v4, v5, v6, v7, v8) => {
            let v2: Rc<UH4> = v2.clone();
            let v3: Rc<UH6> = v3.clone();
            let v4: bool = *v4;
            let mut v5: bool = *v5;
            let v6: bool = *v6;
            let v7: bool = *v7;
            let v8: bool = *v8;
            let mut v9: u32 = 124u32;
            let v10: bool = heads_20(v9, v3.clone());
            if v10 {
                let v11: Rc<UH6> = after_one_44(v3);
                let v12: US3 = rust_alternation_25(v11, v4, v5, v6, v7, v8, v0);
                rust_join_27(v2, v12)
            } else {
                v1
            }
        }
        _ => {
            v1
        }
    }
}
fn is_word_edge_47(mut v0: US8) -> bool {
    match v0 {
        US8::US8_WordEdge => {
            true
        }
        _ => {
            false
        }
    }
}
fn seq_right_51(mut v0: Rc<UH9>, mut v1: US9) -> US9 {
    match v1 {
        US9::US9_Accepts(v2) => {
            let v2: Rc<UH9> = v2;
            let v3: Rc<UH9> = Rc::new(UH9::UH9_SeqAccepts(v0, v2));
            US9::US9_Accepts(v3)
        }
        US9::US9_Rejects(v5) => {
            let mut v5: Rc<UH10> = v5;
            let v6: Rc<UH10> = Rc::new(UH10::UH10_SeqRightRejects(v5));
            US9::US9_Rejects(v6)
        }
    }
}
fn seq_nullability_50(mut v0: US9, mut v1: Rc<UH4>) -> US9 {
    match v0 {
        US9::US9_Accepts(v5) => {
            let mut v5: Rc<UH9> = v5;
            let v6: US9 = nullability_of_49(v1);
            seq_right_51(v5, v6)
        }
        US9::US9_Rejects(v2) => {
            let v2: Rc<UH10> = v2;
            let v3: Rc<UH10> = Rc::new(UH10::UH10_SeqLeftRejects(v2));
            US9::US9_Rejects(v3)
        }
    }
}
fn alt_right_54(mut v0: Rc<UH10>, mut v1: US9) -> US9 {
    match v1 {
        US9::US9_Accepts(v2) => {
            let v2: Rc<UH9> = v2;
            let v3: Rc<UH9> = Rc::new(UH9::UH9_AltRightAccepts(v2));
            US9::US9_Accepts(v3)
        }
        US9::US9_Rejects(v5) => {
            let mut v5: Rc<UH10> = v5;
            let v6: Rc<UH10> = Rc::new(UH10::UH10_AltRejects(v0, v5));
            US9::US9_Rejects(v6)
        }
    }
}
fn alt_nullability_53(mut v0: US9, mut v1: Rc<UH4>) -> US9 {
    match v0 {
        US9::US9_Accepts(v2) => {
            let v2: Rc<UH9> = v2;
            let v3: Rc<UH9> = Rc::new(UH9::UH9_AltLeftAccepts(v2));
            US9::US9_Accepts(v3)
        }
        US9::US9_Rejects(v5) => {
            let mut v5: Rc<UH10> = v5;
            let v6: US9 = nullability_of_49(v1);
            alt_right_54(v5, v6)
        }
    }
}
fn and_right_56(mut v0: Rc<UH9>, mut v1: US9) -> US9 {
    match v1 {
        US9::US9_Accepts(v2) => {
            let v2: Rc<UH9> = v2;
            let v3: Rc<UH9> = Rc::new(UH9::UH9_AndAccepts(v0, v2));
            US9::US9_Accepts(v3)
        }
        US9::US9_Rejects(v5) => {
            let mut v5: Rc<UH10> = v5;
            let v6: Rc<UH10> = Rc::new(UH10::UH10_AndRightRejects(v5));
            US9::US9_Rejects(v6)
        }
    }
}
fn and_nullability_55(mut v0: US9, mut v1: Rc<UH4>) -> US9 {
    match v0 {
        US9::US9_Accepts(v5) => {
            let mut v5: Rc<UH9> = v5;
            let v6: US9 = nullability_of_49(v1);
            and_right_56(v5, v6)
        }
        US9::US9_Rejects(v2) => {
            let v2: Rc<UH10> = v2;
            let v3: Rc<UH10> = Rc::new(UH10::UH10_AndLeftRejects(v2));
            US9::US9_Rejects(v3)
        }
    }
}
fn repeat_null_58(mut v0: Rc<UH4>, mut v1: u32) -> US9 {
    let v2: bool = v1 == 0u32;
    if v2 {
        let v3: Rc<UH9> = { thread_local!{ static CASE: Rc<UH9> = Rc::new(UH9::UH9_RepeatFromZero); } CASE.with(|case| case.clone()) };
        US9::US9_Accepts(v3)
    } else {
        let mut v5: US9 = nullability_of_49(v0);
        match v5 {
            US9::US9_Accepts(v6) => {
                let v6: Rc<UH9> = v6;
                let v7: Rc<UH9> = Rc::new(UH9::UH9_RepeatOfNullable(v6));
                US9::US9_Accepts(v7)
            }
            US9::US9_Rejects(v9) => {
                let mut v9: Rc<UH10> = v9;
                let v10: Rc<UH10> = Rc::new(UH10::UH10_RepeatRejects(v9));
                US9::US9_Rejects(v10)
            }
        }
    }
}
fn nullability_loop_57(mut v0: Rc<UH4>) -> US9 {
    match Rc::unwrap_or_clone(v0) {
        UH4::UH4_Not(v1) => {
            let mut v1: Rc<UH4> = v1;
            let v2: US9 = nullability_of_49(v1);
            match v2 {
                US9::US9_Accepts(v3) => {
                    let v3: Rc<UH9> = v3;
                    let v4: Rc<UH10> = Rc::new(UH10::UH10_NotOfAccepting(v3));
                    US9::US9_Rejects(v4)
                }
                US9::US9_Rejects(v6) => {
                    let v6: Rc<UH10> = v6;
                    let v7: Rc<UH9> = Rc::new(UH9::UH9_NotOfRejecting(v6));
                    US9::US9_Accepts(v7)
                }
            }
        }
        UH4::UH4_Repeat(v14, v15, _) => {
            let v14: Rc<UH4> = v14;
            let v15: u32 = v15;
            repeat_null_58(v14, v15)
        }
        UH4::UH4_Star(_) => {
            let v12: Rc<UH9> = { thread_local!{ static CASE: Rc<UH9> = Rc::new(UH9::UH9_StarAccepts); } CASE.with(|case| case.clone()) };
            US9::US9_Accepts(v12)
        }
        _ => {
            let v18: Rc<UH10> = { thread_local!{ static CASE: Rc<UH10> = Rc::new(UH10::UH10_NothingRejects); } CASE.with(|case| case.clone()) };
            US9::US9_Rejects(v18)
        }
    }
}
fn nullability_rest_52(mut v0: Rc<UH4>) -> US9 {
    match &*v0 {
        UH4::UH4_Alt(v1, v2) => {
            let mut v1: Rc<UH4> = v1.clone();
            let v2: Rc<UH4> = v2.clone();
            let v3: US9 = nullability_of_49(v1);
            alt_nullability_53(v3, v2)
        }
        UH4::UH4_And(v5, v6) => {
            let mut v5: Rc<UH4> = v5.clone();
            let v6: Rc<UH4> = v6.clone();
            let v7: US9 = nullability_of_49(v5);
            and_nullability_55(v7, v6)
        }
        _ => {
            nullability_loop_57(v0)
        }
    }
}
fn nullability_of_49(mut v0: Rc<UH4>) -> US9 {
    match &*v0 {
        UH4::UH4_Epsilon => {
            let v3: Rc<UH9> = { thread_local!{ static CASE: Rc<UH9> = Rc::new(UH9::UH9_EpsilonAccepts); } CASE.with(|case| case.clone()) };
            US9::US9_Accepts(v3)
        }
        UH4::UH4_Nothing => {
            let mut v1: Rc<UH10> = { thread_local!{ static CASE: Rc<UH10> = Rc::new(UH10::UH10_NothingRejects); } CASE.with(|case| case.clone()) };
            US9::US9_Rejects(v1)
        }
        UH4::UH4_Seq(v8, v9) => {
            let v8: Rc<UH4> = v8.clone();
            let mut v9: Rc<UH4> = v9.clone();
            let v10: US9 = nullability_of_49(v8);
            seq_nullability_50(v10, v9)
        }
        UH4::UH4_Set(_) => {
            let v6: Rc<UH10> = { thread_local!{ static CASE: Rc<UH10> = Rc::new(UH10::UH10_SetRejects); } CASE.with(|case| case.clone()) };
            US9::US9_Rejects(v6)
        }
        _ => {
            nullability_rest_52(v0)
        }
    }
}
fn nullable_48(mut v0: Rc<UH4>) -> bool {
    let mut v1: US9 = nullability_of_49(v0);
    match v1 {
        US9::US9_Accepts(_) => {
            true
        }
        US9::US9_Rejects(_) => {
            false
        }
    }
}
fn is_empty_61(mut v0: US4) -> bool {
    let v2: Rc<UH5> = match v0 {
        US4::US4_Normal(v1) => {
            let mut v1: Rc<UH5> = v1;
            v1
        }
    };
    match Rc::unwrap_or_clone(v2) {
        UH5::UH5_Nil => {
            true
        }
        _ => {
            false
        }
    }
}
fn gap_before_64(mut v0: u32, mut v1: u32, mut v2: u32, mut v3: Rc<UH5>) -> Rc<UH5> {
    let v4: bool = v0 > v2;
    if v4 {
        let mut v5: u32 = v0.wrapping_sub(1u32);
        Rc::new(UH5::UH5_Cons(v2, v5, v3))
    } else {
        v3
    }
}
fn gaps_63(mut v0: Rc<UH5>, mut v1: u32) -> Rc<UH5> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v2, v3, v4) => {
            let v2: u32 = v2;
            let v3: u32 = v3;
            let v4: Rc<UH5> = v4;
            let mut v5: bool = v3 >= 1114111u32;
            let mut v9: Rc<UH5> = if v5 {
                { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) }
            } else {
                let v7: u32 = v3.wrapping_add(1u32);
                gaps_63(v4, v7)
            };
            gap_before_64(v2, v3, v1, v9)
        }
        UH5::UH5_Nil => {
            let v11: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH5::UH5_Cons(v1, 1114111u32, v11))
        }
    }
}
fn complement_62(mut v0: US4) -> US4 {
    let v2: Rc<UH5> = match v0 {
        US4::US4_Normal(v1) => {
            let mut v1: Rc<UH5> = v1;
            v1
        }
    };
    let v3: u32 = 0u32;
    let v4: Rc<UH5> = gaps_63(v2, v3);
    US4::US4_Normal(v4)
}
fn star_66(mut v0: Rc<UH4>) -> Rc<UH4> {
    match &*v0 {
        UH4::UH4_Epsilon => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
        }
        UH4::UH4_Nothing => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
        }
        UH4::UH4_Star(_) => {
            v0
        }
        _ => {
            Rc::new(UH4::UH4_Star(v0))
        }
    }
}
fn any_run_65() -> Rc<UH4> {
    let mut v0: u32 = 0u32;
    let mut v1: u32 = 1114111u32;
    let v2: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH5> = Rc::new(UH5::UH5_Cons(v0, v1, v2));
    let v4: US4 = US4::US4_Normal(v3);
    let mut v5: Rc<UH4> = Rc::new(UH4::UH4_Set(v4));
    star_66(v5)
}
fn same_re_69(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> bool {
    let v2: US7 = order_re_32(v0, v1);
    match v2 {
        US7::US7_Same => {
            true
        }
        _ => {
            false
        }
    }
}
fn starred_twice_68(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> bool {
    match &*v0 {
        UH4::UH4_Star(_) => {
            match &*v1 {
                UH4::UH4_Seq(v3, _) => {
                    let v3: Rc<UH4> = v3.clone();
                    same_re_69(v0, v3)
                }
                _ => {
                    same_re_69(v0, v1)
                }
            }
        }
        _ => {
            false
        }
    }
}
fn seq_67(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v0 {
            UH4::UH4_Epsilon => {
                return v1;
            }
            UH4::UH4_Nothing => {
                return { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) };
            }
            UH4::UH4_Seq(v3, v4) => {
                let v3: Rc<UH4> = v3.clone();
                let v4: Rc<UH4> = v4.clone();
                let mut v5: Rc<UH4> = seq_67(v4, v1);
                (v0, v1) = (v3, v5);
                continue;
            }
            _ => {
                match &*v1 {
                    UH4::UH4_Epsilon => {
                        return v0;
                    }
                    UH4::UH4_Nothing => {
                        return { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) };
                    }
                    _ => {
                        let v8: bool = starred_twice_68(v0.clone(), v1.clone());
                        if v8 {
                            return v1;
                        } else {
                            return Rc::new(UH4::UH4_Seq(v0, v1));
                        }
                    }
                }
            }
        }
    }
}
fn and_of_71(mut v0: Rc<UH8>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v0) {
        UH8::UH8_Cons(v3, v4) => {
            let v3: Rc<UH4> = v3;
            let v4: Rc<UH8> = v4;
            match &*v4 {
                UH8::UH8_Nil => {
                    v3
                }
                _ => {
                    let mut v5: Rc<UH4> = and_of_71(v4);
                    Rc::new(UH4::UH4_And(v3, v5))
                }
            }
        }
        UH8::UH8_Nil => {
            let mut v1: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) };
            Rc::new(UH4::UH4_Not(v1))
        }
    }
}
fn and_leaves_72(mut v0: Rc<UH4>, mut v1: Rc<UH8>) -> Rc<UH8> {
    loop {
        match &*v0 {
            UH4::UH4_And(v2, v3) => {
                let v2: Rc<UH4> = v2.clone();
                let v3: Rc<UH4> = v3.clone();
                let v4: Rc<UH8> = and_leaves_72(v3, v1);
                (v0, v1) = (v2, v4);
                continue;
            }
            _ => {
                let v8: bool = match &*v0 {
                    UH4::UH4_Not(v6) => {
                        let v6: Rc<UH4> = v6.clone();
                        match Rc::unwrap_or_clone(v6) {
                            UH4::UH4_Nothing => {
                                true
                            }
                            _ => {
                                false
                            }
                        }
                    }
                    _ => {
                        false
                    }
                };
                if v8 {
                    return v1;
                } else {
                    return insert_leaf_31(v0, v1);
                }
            }
        }
    }
}
fn both_70(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    let v2: bool = match &*v0 {
        UH4::UH4_Nothing => {
            true
        }
        _ => {
            false
        }
    };
    let v4: bool = if v2 {
        true
    } else {
        match &*v1 {
            UH4::UH4_Nothing => {
                true
            }
            _ => {
                false
            }
        }
    };
    if v4 {
        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
    } else {
        let v6: Rc<UH8> = { thread_local!{ static CASE: Rc<UH8> = Rc::new(UH8::UH8_Nil); } CASE.with(|case| case.clone()) };
        let v7: Rc<UH8> = and_leaves_72(v1, v6);
        let v8: Rc<UH8> = and_leaves_72(v0, v7);
        and_of_71(v8)
    }
}
fn word_left_60(mut v0: US4, mut v1: Rc<UH4>) -> Rc<UH4> {
    let v2: bool = is_empty_61(v0.clone());
    let mut v5: Rc<UH4> = if v2 {
        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
    } else {
        Rc::new(UH4::UH4_Set(v0.clone()))
    };
    let v6: US4 = complement_62(v0);
    let v7: bool = is_empty_61(v6.clone());
    let v10: Rc<UH4> = if v7 {
        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
    } else {
        Rc::new(UH4::UH4_Set(v6))
    };
    let v11: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH4> = any_run_65();
    let v13: Rc<UH4> = seq_67(v12, v10.clone());
    let v14: Rc<UH4> = alt_28(v11, v13);
    let v15: Rc<UH4> = any_run_65();
    let v16: Rc<UH4> = seq_67(v5.clone(), v15);
    let v17: Rc<UH4> = both_70(v1.clone(), v16);
    let v18: Rc<UH4> = seq_67(v14, v17);
    let v19: Rc<UH4> = any_run_65();
    let v20: Rc<UH4> = seq_67(v19, v5);
    let v21: Rc<UH4> = any_run_65();
    let v22: Rc<UH4> = seq_67(v10, v21);
    let v23: Rc<UH4> = both_70(v1, v22);
    let v24: Rc<UH4> = seq_67(v20, v23);
    alt_28(v18, v24)
}
fn number_from_78(mut v0: Rc<UH7>, mut v1: u32) -> u32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH7::UH7_Cons(v2, v3) => {
                let v2: i32 = v2;
                let v3: Rc<UH7> = v3;
                let v4: u32 = v1.wrapping_mul(10u32);
                let mut v5: i32 = v2.wrapping_sub(48i32);
                let v6: u32 = (v5 as u32);
                let v7: u32 = v4.wrapping_add(v6);
                (v0, v1) = (v3, v7);
                continue;
            }
            UH7::UH7_Nil => {
                return v1;
            }
        }
    }
}
fn method77(mut v0: Rc<UH0>, mut v1: Rc<UH6>) -> Rc<UH6> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH6> = method77(v3, v1);
            let mut v5: Rc<Vec<char>> = Rc::new((&*v2).chars().collect());
            let v6: Rc<Vec<char>> = Rc::new((v5).as_ref().iter().rev().cloned().collect::<Vec<_>>());
            let v7: usize = (v6).len();
            let v8: i32 = ((v7) as i32);
            let mut v9: i32 = 0i32;
            let v10: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
            let v11: Rc<UH7> = method23(v6, v8, v9, v10);
            let v12: u32 = 0u32;
            let v13: u32 = number_from_78(v11, v12);
            Rc::new(UH6::UH6_Cons(v13, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method80(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_79(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method80(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v16, v17)
    }
}
fn numbers_of_76(mut v0: Rc<str>) -> Rc<UH6> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_79(v1, v0);
    let v3: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    method77(v2, v3)
}
fn ranges_from_81(mut v0: Rc<UH6>, mut v1: Rc<UH6>, mut v2: Rc<UH5>) -> Rc<UH5> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v3, v4) => {
                let v3: u32 = v3;
                let v4: Rc<UH6> = v4;
                match Rc::unwrap_or_clone(v1) {
                    UH6::UH6_Cons(v5, v6) => {
                        let mut v5: u32 = v5;
                        let v6: Rc<UH6> = v6;
                        let v7: Rc<UH5> = Rc::new(UH5::UH5_Cons(v3, v5, v2));
                        (v0, v1, v2) = (v4, v6, v7);
                        continue;
                    }
                    _ => {
                        return v2;
                    }
                }
            }
            _ => {
                return v2;
            }
        }
    }
}
fn insert_by_low_83(mut v0: u32, mut v1: u32, mut v2: Rc<UH5>) -> Rc<UH5> {
    match &*v2 {
        UH5::UH5_Cons(v3, v4, v5) => {
            let v3: u32 = *v3;
            let v4: u32 = *v4;
            let mut v5: Rc<UH5> = v5.clone();
            let v6: bool = v0 <= v3;
            if v6 {
                Rc::new(UH5::UH5_Cons(v0, v1, v2))
            } else {
                let v8: Rc<UH5> = insert_by_low_83(v0, v1, v5);
                Rc::new(UH5::UH5_Cons(v3, v4, v8))
            }
        }
        UH5::UH5_Nil => {
            let v11: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH5::UH5_Cons(v0, v1, v11))
        }
    }
}
fn sorted_by_low_82(mut v0: Rc<UH5>, mut v1: Rc<UH5>) -> Rc<UH5> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH5::UH5_Cons(v2, v3, v4) => {
                let v2: u32 = v2;
                let v3: u32 = v3;
                let v4: Rc<UH5> = v4;
                let mut v5: bool = v2 <= v3;
                let v7: Rc<UH5> = if v5 {
                    insert_by_low_83(v2, v3, v1)
                } else {
                    v1
                };
                (v0, v1) = (v4, v7);
                continue;
            }
            UH5::UH5_Nil => {
                return v1;
            }
        }
    }
}
fn merge_pair_85(mut v0: u32, mut v1: u32, mut v2: u32, mut v3: u32, mut v4: Rc<UH5>) -> Rc<UH5> {
    let mut v5: u32 = v1.wrapping_add(1u32);
    let v6: bool = v2 <= v5;
    if v6 {
        let v7: bool = v1 < v3;
        let v8: u32 = if v7 {
            v3
        } else {
            v1
        };
        Rc::new(UH5::UH5_Cons(v0, v8, v4))
    } else {
        let v10: Rc<UH5> = Rc::new(UH5::UH5_Cons(v2, v3, v4));
        Rc::new(UH5::UH5_Cons(v0, v1, v10))
    }
}
fn merged_84(mut v0: Rc<UH5>) -> Rc<UH5> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v1, v2, v3) => {
            let mut v1: u32 = v1;
            let v2: u32 = v2;
            let v3: Rc<UH5> = v3;
            let v4: Rc<UH5> = merged_84(v3);
            match Rc::unwrap_or_clone(v4) {
                UH5::UH5_Cons(v5, v6, v7) => {
                    let mut v5: u32 = v5;
                    let v6: u32 = v6;
                    let v7: Rc<UH5> = v7;
                    merge_pair_85(v1, v2, v5, v6, v7)
                }
                UH5::UH5_Nil => {
                    let mut v9: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH5::UH5_Cons(v1, v2, v9))
                }
            }
        }
        UH5::UH5_Nil => {
            { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn table_set_75(mut v0: Rc<str>, mut v1: Rc<str>) -> US4 {
    let v2: Rc<UH6> = numbers_of_76(v0);
    let v3: Rc<UH6> = numbers_of_76(v1);
    let v4: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH5> = ranges_from_81(v2, v3, v4);
    let v6: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH5> = sorted_by_low_82(v5, v6);
    let v8: Rc<UH5> = merged_84(v7);
    US4::US4_Normal(v8)
}
fn unicode_word_74() -> US4 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("48,65,95,97,170,181,186,192,216,248,710,736,748,750,768,886,890,895,902,904,908,910,931,1015,1155,1329,1369,1376,1425,1471,1473,1476,1479,1488,1519,1552,1568,1646,1749,1759,1770,1791,1808,1869,1984,2042,2045,2048,2112,2144,2160,2185,2199,2275,2406,2417,2437,2447,2451,2474,2482,2486,2492,2503,2507,2519,2524,2527,2534,2556,2558,2561,2565,2575,2579,2602,2610,2613,2616,2620,2622,2631,2635,2641,2649,2654,2662,2689,2693,2703,2707,2730,2738,2741,2748,2759,2763,2768,2784,2790,2809,2817,2821,2831,2835,2858,2866,2869,2876,2887,2891,2901,2908,2911,2918,2929,2946,2949,2958,2962,2969,2972,2974,2979,2984,2990,3006,3014,3018,3024,3031,3046,3072,3086,3090,3114,3132,3142,3146,3157,3160,3165,3168,3174,3200,3205,3214,3218,3242,3253,3260,3270,3274,3285,3293,3296,3302,3313,3328,3342,3346,3398,3402,3412,3423,3430,3450,3457,3461,3482,3507,3517,3520,3530,3535,3542,3544,3558,3570,3585,3648,3664,3713,3716,3718,3724,3749,3751,3776,3782,3784,3792,3804,3840,3864,3872,3893,3895,3897,3902,3913,3953,3974,3993,4038,4096,4176,4256,4295,4301,4304,4348,4682,4688,4696,4698,4704,4746,4752,4786,4792,4800,4802,4808,4824,4882,4888,4957,4992,5024,5112,5121,5743,5761,5792,5870,5888,5919,5952,5984,5998,6002,6016,6103,6108,6112,6155,6159,6176,6272,6320,6400,6432,6448,6470,6512,6528,6576,6608,6656,6688,6752,6783,6800,6823,6832,6912,6992,7019,7040,7168,7232,7245,7296,7312,7357,7376,7380,7424,7960,7968,8008,8016,8025,8027,8029,8031,8064,8118,8126,8130,8134,8144,8150,8160,8178,8182,8204,8255,8276,8305,8319,8336,8400,8450,8455,8458,8469,8473,8484,8486,8488,8490,8495,8508,8517,8526,8544,9398,11264,11499,11520,11559,11565,11568,11631,11647,11680,11688,11696,11704,11712,11720,11728,11736,11744,11823,12293,12321,12337,12344,12353,12441,12445,12449,12540,12549,12593,12704,12784,13312,19968,42192,42240,42512,42560,42612,42623,42775,42786,42891,42960,42963,42965,42994,43052,43072,43136,43216,43232,43259,43261,43312,43360,43392,43471,43488,43520,43584,43600,43616,43642,43739,43744,43762,43777,43785,43793,43808,43816,43824,43868,43888,44012,44016,44032,55216,55243,63744,64112,64256,64275,64285,64298,64312,64318,64320,64323,64326,64467,64848,64914,65008,65024,65056,65075,65101,65136,65142,65296,65313,65343,65345,65382,65474,65482,65490,65498,65536,65549,65576,65596,65599,65616,65664,65856,66045,66176,66208,66272,66304,66349,66384,66432,66464,66504,66513,66560,66720,66736,66776,66816,66864,66928,66940,66956,66964,66967,66979,66995,67003,67008,67072,67392,67424,67456,67463,67506,67584,67592,67594,67639,67644,67647,67680,67712,67808,67828,67840,67872,67968,68030,68096,68101,68108,68117,68121,68152,68159,68192,68224,68288,68297,68352,68416,68448,68480,68608,68736,68800,68864,68912,68928,68969,68975,69248,69291,69296,69314,69372,69415,69424,69488,69552,69600,69632,69734,69759,69826,69840,69872,69888,69942,69956,69968,70006,70016,70089,70094,70108,70144,70163,70206,70272,70280,70282,70287,70303,70320,70384,70400,70405,70415,70419,70442,70450,70453,70459,70471,70475,70480,70487,70493,70502,70512,70528,70539,70542,70544,70583,70594,70597,70599,70604,70625,70656,70736,70750,70784,70855,70864,71040,71096,71128,71168,71236,71248,71296,71360,71376,71424,71453,71472,71488,71680,71840,71935,71945,71948,71957,71960,71991,71995,72016,72096,72106,72154,72163,72192,72263,72272,72349,72368,72640,72688,72704,72714,72760,72784,72818,72850,72873,72960,72968,72971,73018,73020,73023,73040,73056,73063,73066,73104,73107,73120,73440,73472,73490,73534,73552,73648,73728,74752,74880,77712,77824,78912,78944,82944,90368,92160,92736,92768,92784,92864,92880,92912,92928,92992,93008,93027,93053,93504,93552,93760,93952,94031,94095,94176,94179,94192,94208,100352,101631,110576,110581,110589,110592,110898,110928,110933,110948,110960,113664,113776,113792,113808,113821,118000,118528,118576,119141,119149,119163,119173,119210,119362,119808,119894,119966,119970,119973,119977,119982,119995,119997,120005,120071,120077,120086,120094,120123,120128,120134,120138,120146,120488,120514,120540,120572,120598,120630,120656,120688,120714,120746,120772,120782,121344,121403,121461,121476,121499,121505,122624,122661,122880,122888,122907,122915,122918,122928,123023,123136,123184,123200,123214,123536,123584,124112,124368,124896,124904,124909,124912,124928,125136,125184,125264,126464,126469,126497,126500,126503,126505,126516,126521,126523,126530,126535,126537,126539,126541,126545,126548,126551,126553,126555,126557,126559,126561,126564,126567,126572,126580,126585,126590,126592,126603,126625,126629,126635,127280,127312,127344,130032,131072,173824,177984,178208,183984,191472,194560,196608,201552,917760"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("57,90,95,122,170,181,186,214,246,705,721,740,748,750,884,887,893,895,902,906,908,929,1013,1153,1327,1366,1369,1416,1469,1471,1474,1477,1479,1514,1522,1562,1641,1747,1756,1768,1788,1791,1866,1969,2037,2042,2045,2093,2139,2154,2183,2190,2273,2403,2415,2435,2444,2448,2472,2480,2482,2489,2500,2504,2510,2519,2525,2531,2545,2556,2558,2563,2570,2576,2600,2608,2611,2614,2617,2620,2626,2632,2637,2641,2652,2654,2677,2691,2701,2705,2728,2736,2739,2745,2757,2761,2765,2768,2787,2799,2815,2819,2828,2832,2856,2864,2867,2873,2884,2888,2893,2903,2909,2915,2927,2929,2947,2954,2960,2965,2970,2972,2975,2980,2986,3001,3010,3016,3021,3024,3031,3055,3084,3088,3112,3129,3140,3144,3149,3158,3162,3165,3171,3183,3203,3212,3216,3240,3251,3257,3268,3272,3277,3286,3294,3299,3311,3315,3340,3344,3396,3400,3406,3415,3427,3439,3455,3459,3478,3505,3515,3517,3526,3530,3540,3542,3551,3567,3571,3642,3662,3673,3714,3716,3722,3747,3749,3773,3780,3782,3790,3801,3807,3840,3865,3881,3893,3895,3897,3911,3948,3972,3991,4028,4038,4169,4253,4293,4295,4301,4346,4680,4685,4694,4696,4701,4744,4749,4784,4789,4798,4800,4805,4822,4880,4885,4954,4959,5007,5109,5117,5740,5759,5786,5866,5880,5909,5940,5971,5996,6000,6003,6099,6103,6109,6121,6157,6169,6264,6314,6389,6430,6443,6459,6509,6516,6571,6601,6617,6683,6750,6780,6793,6809,6823,6862,6988,7001,7027,7155,7223,7241,7293,7306,7354,7359,7378,7418,7957,7965,8005,8013,8023,8025,8027,8029,8061,8116,8124,8126,8132,8140,8147,8155,8172,8180,8188,8205,8256,8276,8305,8319,8348,8432,8450,8455,8467,8469,8477,8484,8486,8488,8493,8505,8511,8521,8526,8584,9449,11492,11507,11557,11559,11565,11623,11631,11670,11686,11694,11702,11710,11718,11726,11734,11742,11775,11823,12295,12335,12341,12348,12438,12442,12447,12538,12543,12591,12686,12735,12799,19903,42124,42237,42508,42539,42610,42621,42737,42783,42888,42957,42961,42963,42972,43047,43052,43123,43205,43225,43255,43259,43309,43347,43388,43456,43481,43518,43574,43597,43609,43638,43714,43741,43759,43766,43782,43790,43798,43814,43822,43866,43881,44010,44013,44025,55203,55238,55291,64109,64217,64262,64279,64296,64310,64316,64318,64321,64324,64433,64829,64911,64967,65019,65039,65071,65076,65103,65140,65276,65305,65338,65343,65370,65470,65479,65487,65495,65500,65547,65574,65594,65597,65613,65629,65786,65908,66045,66204,66256,66272,66335,66378,66426,66461,66499,66511,66517,66717,66729,66771,66811,66855,66915,66938,66954,66962,66965,66977,66993,67001,67004,67059,67382,67413,67431,67461,67504,67514,67589,67592,67637,67640,67644,67669,67702,67742,67826,67829,67861,67897,68023,68031,68099,68102,68115,68119,68149,68154,68159,68220,68252,68295,68326,68405,68437,68466,68497,68680,68786,68850,68903,68921,68965,68973,68997,69289,69292,69297,69316,69404,69415,69456,69509,69572,69622,69702,69749,69818,69826,69864,69881,69940,69951,69959,70003,70006,70084,70092,70106,70108,70161,70199,70209,70278,70280,70285,70301,70312,70378,70393,70403,70412,70416,70440,70448,70451,70457,70468,70472,70477,70480,70487,70499,70508,70516,70537,70539,70542,70581,70592,70594,70597,70602,70611,70626,70730,70745,70753,70853,70855,70873,71093,71104,71133,71232,71236,71257,71352,71369,71395,71450,71467,71481,71494,71738,71913,71942,71945,71955,71958,71989,71992,72003,72025,72103,72151,72161,72164,72254,72263,72345,72349,72440,72672,72697,72712,72758,72768,72793,72847,72871,72886,72966,72969,73014,73018,73021,73031,73049,73061,73064,73102,73105,73112,73129,73462,73488,73530,73538,73562,73648,74649,74862,75075,77808,78895,78933,82938,83526,90425,92728,92766,92777,92862,92873,92909,92916,92982,92995,93017,93047,93071,93548,93561,93823,94026,94087,94111,94177,94180,94193,100343,101589,101640,110579,110587,110590,110882,110898,110930,110933,110951,111355,113770,113788,113800,113817,113822,118009,118573,118598,119145,119154,119170,119179,119213,119364,119892,119964,119967,119970,119974,119980,119993,119995,120003,120069,120074,120084,120092,120121,120126,120132,120134,120144,120485,120512,120538,120570,120596,120628,120654,120686,120712,120744,120770,120779,120831,121398,121452,121461,121476,121503,121519,122654,122666,122886,122904,122913,122916,122922,122989,123023,123180,123197,123209,123214,123566,123641,124153,124410,124902,124907,124910,124926,125124,125142,125259,125273,126467,126495,126498,126500,126503,126514,126519,126521,126523,126530,126535,126537,126539,126543,126546,126548,126551,126553,126555,126557,126559,126562,126564,126570,126578,126583,126588,126590,126601,126619,126627,126633,126651,127305,127337,127369,130041,173791,177977,178205,183969,191456,192093,195101,201546,205743,917999"); } LIT.with(|lit| lit.clone()) };
    table_set_75(v0, v1)
}
fn ascii_word_86() -> US4 {
    let mut v0: u32 = 48u32;
    let mut v1: u32 = 57u32;
    let v2: u32 = 65u32;
    let v3: u32 = 90u32;
    let v4: u32 = 97u32;
    let mut v5: u32 = 122u32;
    let v6: u32 = 95u32;
    let v7: u32 = 95u32;
    let v8: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH5> = Rc::new(UH5::UH5_Cons(v6, v7, v8));
    let v10: Rc<UH5> = Rc::new(UH5::UH5_Cons(v4, v5, v9));
    let v11: Rc<UH5> = Rc::new(UH5::UH5_Cons(v2, v3, v10));
    let v12: Rc<UH5> = Rc::new(UH5::UH5_Cons(v0, v1, v11));
    let v13: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH5> = sorted_by_low_82(v12, v13);
    let v15: Rc<UH5> = merged_84(v14);
    US4::US4_Normal(v15)
}
fn word_set_73(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool) -> US4 {
    if v3 {
        unicode_word_74()
    } else {
        ascii_word_86()
    }
}
fn word_right_87(mut v0: US4, mut v1: Rc<UH4>) -> Rc<UH4> {
    let v2: bool = is_empty_61(v0.clone());
    let mut v5: Rc<UH4> = if v2 {
        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
    } else {
        Rc::new(UH4::UH4_Set(v0.clone()))
    };
    let v6: US4 = complement_62(v0);
    let v7: bool = is_empty_61(v6.clone());
    let v10: Rc<UH4> = if v7 {
        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
    } else {
        Rc::new(UH4::UH4_Set(v6))
    };
    let v11: Rc<UH4> = any_run_65();
    let v12: Rc<UH4> = seq_67(v11, v5.clone());
    let v13: Rc<UH4> = both_70(v1.clone(), v12);
    let v14: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH4> = any_run_65();
    let v16: Rc<UH4> = seq_67(v10.clone(), v15);
    let v17: Rc<UH4> = alt_28(v14, v16);
    let v18: Rc<UH4> = seq_67(v13, v17);
    let v19: Rc<UH4> = any_run_65();
    let v20: Rc<UH4> = seq_67(v19, v10);
    let v21: Rc<UH4> = both_70(v1, v20);
    let v22: Rc<UH4> = any_run_65();
    let v23: Rc<UH4> = seq_67(v5, v22);
    let v24: Rc<UH4> = seq_67(v21, v23);
    alt_28(v18, v24)
}
fn right_pad_88(mut v0: US8) -> Rc<UH4> {
    match v0 {
        US8::US8_PinnedLine => {
            let v3: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
            let v4: u32 = 10u32;
            let mut v5: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
            let v6: Rc<UH5> = Rc::new(UH5::UH5_Cons(v4, v4, v5));
            let v7: US4 = US4::US4_Normal(v6);
            let v8: Rc<UH4> = Rc::new(UH4::UH4_Set(v7));
            let mut v9: Rc<UH4> = any_run_65();
            let v10: Rc<UH4> = seq_67(v8, v9);
            alt_28(v3, v10)
        }
        US8::US8_PinnedText => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
        }
        US8::US8_Unpinned => {
            any_run_65()
        }
        US8::US8_WordEdge => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
        }
    }
}
fn left_pad_89(mut v0: US8) -> Rc<UH4> {
    match v0 {
        US8::US8_PinnedLine => {
            let v3: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
            let v4: Rc<UH4> = any_run_65();
            let mut v5: u32 = 10u32;
            let v6: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
            let v7: Rc<UH5> = Rc::new(UH5::UH5_Cons(v5, v5, v6));
            let v8: US4 = US4::US4_Normal(v7);
            let mut v9: Rc<UH4> = Rc::new(UH4::UH4_Set(v8));
            let v10: Rc<UH4> = seq_67(v4, v9);
            alt_28(v3, v10)
        }
        US8::US8_PinnedText => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
        }
        US8::US8_Unpinned => {
            any_run_65()
        }
        US8::US8_WordEdge => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
        }
    }
}
fn rust_finish_59(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: US8, mut v7: US8, mut v8: Rc<UH4>) -> Rc<UH4> {
    let mut v9: bool = v5 == false;
    if v9 {
        v8
    } else {
        let v10: US4 = word_set_73(v0, v1, v2, v3, v4);
        let v11: bool = is_word_edge_47(v7.clone());
        let v15: Rc<UH4> = if v11 {
            word_right_87(v10.clone(), v8)
        } else {
            let v13: Rc<UH4> = right_pad_88(v7);
            seq_67(v8, v13)
        };
        let v16: bool = is_word_edge_47(v6.clone());
        if v16 {
            word_left_60(v10, v15)
        } else {
            let v18: Rc<UH4> = left_pad_89(v6);
            seq_67(v18, v15)
        }
    }
}
fn rust_close_46(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: US8, mut v7: US8, mut v8: Rc<UH4>, mut v9: Rc<UH6>) -> US3 {
    let v10: bool = is_word_edge_47(v6.clone());
    let v12: bool = if v10 {
        true
    } else {
        is_word_edge_47(v7.clone())
    };
    let v14: bool = if v12 {
        nullable_48(v8.clone())
    } else {
        false
    };
    if v14 {
        let v15: US2 = US2::US2_WordBoundaryUnsupported;
        US3::US3_Stopped(v15)
    } else {
        let v17: Rc<UH4> = rust_finish_59(v0, v1, v2, v3, v4, v5, v6, v7, v8);
        US3::US3_Got(v17, v9, v0, v1, v2, v3, v4)
    }
}
fn word_between_93(mut v0: US4, mut v1: Rc<UH4>, mut v2: Rc<UH4>) -> Rc<UH4> {
    let v3: bool = is_empty_61(v0.clone());
    let v6: Rc<UH4> = if v3 {
        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
    } else {
        Rc::new(UH4::UH4_Set(v0.clone()))
    };
    let v7: US4 = complement_62(v0);
    let v8: bool = is_empty_61(v7.clone());
    let v11: Rc<UH4> = if v8 {
        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
    } else {
        Rc::new(UH4::UH4_Set(v7))
    };
    let v12: Rc<UH4> = any_run_65();
    let v13: Rc<UH4> = seq_67(v12, v6.clone());
    let v14: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH4> = any_run_65();
    let v16: Rc<UH4> = seq_67(v15, v11.clone());
    let v17: Rc<UH4> = alt_28(v14, v16);
    let v18: Rc<UH4> = any_run_65();
    let v19: Rc<UH4> = seq_67(v6, v18);
    let v20: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH4> = any_run_65();
    let v22: Rc<UH4> = seq_67(v11, v21);
    let v23: Rc<UH4> = alt_28(v20, v22);
    let v24: Rc<UH4> = both_70(v1.clone(), v13);
    let v25: Rc<UH4> = both_70(v2.clone(), v23);
    let v26: Rc<UH4> = seq_67(v24, v25);
    let v27: Rc<UH4> = both_70(v1, v17);
    let v28: Rc<UH4> = both_70(v2, v19);
    let v29: Rc<UH4> = seq_67(v27, v28);
    alt_28(v26, v29)
}
fn rust_mid_boundary_92(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US8, mut v6: Rc<UH4>, mut v7: US3) -> US3 {
    match &v7 {
        US3::US3_Got(v8, v9, v10, v11, v12, v13, v14) => {
            let v8: Rc<UH4> = v8.clone();
            let mut v9: Rc<UH6> = v9.clone();
            let v10: bool = *v10;
            let v11: bool = *v11;
            let v12: bool = *v12;
            let v13: bool = *v13;
            let v14: bool = *v14;
            let v15: US4 = word_set_73(v0, v1, v2, v3, v4);
            let v16: bool = is_word_edge_47(v5.clone());
            let v20: Rc<UH4> = if v16 {
                word_left_60(v15.clone(), v6)
            } else {
                let v18: Rc<UH4> = left_pad_89(v5);
                seq_67(v18, v6)
            };
            let v21: Rc<UH4> = word_between_93(v15, v20, v8);
            US3::US3_Got(v21, v9, v10, v11, v12, v13, v14)
        }
        _ => {
            v7
        }
    }
}
fn is_space_code_96(mut v0: u32) -> bool {
    let mut v1: bool = 9u32 <= v0;
    let v3: bool = if v1 {
        let v2: bool = v0 <= 13u32;
        v2
    } else {
        false
    };
    if v3 {
        true
    } else {
        let v4: bool = v0 == 32u32;
        v4
    }
}
fn skip_comment_97(mut v0: Rc<UH6>) -> Rc<UH6> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v1, v2) => {
                let mut v1: u32 = v1;
                let v2: Rc<UH6> = v2;
                let v3: bool = v1 == 10u32;
                if v3 {
                    return v2;
                } else {
                    v0 = v2;
                    continue;
                }
            }
            UH6::UH6_Nil => {
                return { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn skip_verbose_95(mut v0: Rc<UH6>) -> Rc<UH6> {
    loop {
        match &*v0 {
            UH6::UH6_Cons(v1, v2) => {
                let mut v1: u32 = *v1;
                let v2: Rc<UH6> = v2.clone();
                let v3: bool = is_space_code_96(v1);
                if v3 {
                    v0 = v2;
                    continue;
                } else {
                    let mut v5: bool = v1 == 35u32;
                    if v5 {
                        let v6: Rc<UH6> = skip_comment_97(v2);
                        v0 = v6;
                        continue;
                    } else {
                        return v0;
                    }
                }
            }
            UH6::UH6_Nil => {
                return { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn skip_space_94(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH6>) -> Rc<UH6> {
    if v4 {
        skip_verbose_95(v5)
    } else {
        v5
    }
}
fn rust_ends_branch_98(mut v0: Rc<UH6>) -> bool {
    match Rc::unwrap_or_clone(v0) {
        UH6::UH6_Cons(v1, _) => {
            let mut v1: u32 = v1;
            let v3: bool = v1 == 124u32;
            if v3 {
                true
            } else {
                let v4: bool = v1 == 41u32;
                v4
            }
        }
        UH6::UH6_Nil => {
            true
        }
    }
}
fn anchor_of_99(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: &Rc<UH6>) -> US10 {
    let v8: US8 = if v2 {
        US8::US8_PinnedLine
    } else {
        US8::US8_PinnedText
    };
    match &**v5 {
        UH6::UH6_Cons(v9, v10) => {
            let mut v9: u32 = *v9;
            let v10: Rc<UH6> = v10.clone();
            let v11: bool = v9 == 94u32;
            if v11 {
                US10::US10_StartAnchor(v8, v10)
            } else {
                let v13: bool = v9 == 36u32;
                if v13 {
                    US10::US10_EndAnchor(v8, v10)
                } else {
                    let v15: bool = v9 == 92u32;
                    let v18: bool = if v15 {
                        let v16: u32 = 65u32;
                        heads_20(v16, v10.clone())
                    } else {
                        false
                    };
                    if v18 {
                        let v19: Rc<UH6> = after_one_44(v10);
                        let v20: US8 = US8::US8_PinnedText;
                        US10::US10_StartAnchor(v20, v19)
                    } else {
                        let v24: bool = if v15 {
                            let v22: u32 = 122u32;
                            heads_20(v22, v10.clone())
                        } else {
                            false
                        };
                        if v24 {
                            let v25: Rc<UH6> = after_one_44(v10);
                            let v26: US8 = US8::US8_PinnedText;
                            US10::US10_EndAnchor(v26, v25)
                        } else {
                            let v30: bool = if v15 {
                                let v28: u32 = 98u32;
                                heads_20(v28, v10.clone())
                            } else {
                                false
                            };
                            if v30 {
                                let v31: Rc<UH6> = after_one_44(v10);
                                US10::US10_WordAnchor(v31)
                            } else {
                                US10::US10_NoAnchor
                            }
                        }
                    }
                }
            }
        }
        UH6::UH6_Nil => {
            US10::US10_NoAnchor
        }
    }
}
fn is_unpinned_100(mut v0: US8) -> bool {
    match v0 {
        US8::US8_Unpinned => {
            true
        }
        _ => {
            false
        }
    }
}
fn flag_set_103(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: u32, mut v6: bool) -> US12 {
    let v7: bool = v5 == 105u32;
    if v7 {
        US12::US12_Some(v6, v1, v2, v3, v4)
    } else {
        let mut v9: bool = v5 == 109u32;
        if v9 {
            US12::US12_Some(v0, v1, v6, v3, v4)
        } else {
            let v11: bool = v5 == 115u32;
            if v11 {
                US12::US12_Some(v0, v6, v2, v3, v4)
            } else {
                let v13: bool = v5 == 117u32;
                if v13 {
                    US12::US12_Some(v0, v1, v2, v6, v4)
                } else {
                    let v15: bool = v5 == 120u32;
                    if v15 {
                        US12::US12_Some(v0, v1, v2, v3, v6)
                    } else {
                        let v17: bool = v5 == 85u32;
                        if v17 {
                            US12::US12_Some(v0, v1, v2, v3, v4)
                        } else {
                            US12::US12_None
                        }
                    }
                }
            }
        }
    }
}
fn scan_flags_102(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: bool, mut v7: Rc<UH6>) -> US11 {
    loop {
        match Rc::unwrap_or_clone(v7) {
            UH6::UH6_Cons(v8, v9) => {
                let v8: u32 = v8;
                let mut v9: Rc<UH6> = v9;
                let v10: bool = v8 == 41u32;
                if v10 {
                    if v6 {
                        return US11::US11_FlagsSet(v0, v1, v2, v3, v4, v9);
                    } else {
                        return US11::US11_FlagsBad;
                    }
                } else {
                    let v14: bool = v8 == 58u32;
                    if v14 {
                        return US11::US11_FlagsScoped(v0, v1, v2, v3, v4, v9);
                    } else {
                        let v16: bool = v8 == 45u32;
                        if v16 {
                            if v5 {
                                let v17: bool = false;
                                (v5, v7) = (v17, v9);
                                continue;
                            } else {
                                return US11::US11_FlagsBad;
                            }
                        } else {
                            let v21: US12 = flag_set_103(v0, v1, v2, v3, v4, v8, v5);
                            match v21 {
                                US12::US12_None => {
                                    return US11::US11_FlagsBad;
                                }
                                US12::US12_Some(v22, v23, v24, v25, v26) => {
                                    let v22: bool = v22;
                                    let v23: bool = v23;
                                    let v24: bool = v24;
                                    let v25: bool = v25;
                                    let v26: bool = v26;
                                    let v27: bool = true;
                                    (v0, v1, v2, v3, v4, v6, v7) = (v22, v23, v24, v25, v26, v27, v9);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            UH6::UH6_Nil => {
                return US11::US11_FlagsBad;
            }
        }
    }
}
fn is_flag_start_104(mut v0: u32) -> bool {
    let mut v1: bool = v0 == 105u32;
    let v3: bool = if v1 {
        true
    } else {
        let v2: bool = v0 == 109u32;
        v2
    };
    let mut v5: bool = if v3 {
        true
    } else {
        let v4: bool = v0 == 115u32;
        v4
    };
    let v7: bool = if v5 {
        true
    } else {
        let v6: bool = v0 == 117u32;
        v6
    };
    let mut v9: bool = if v7 {
        true
    } else {
        let v8: bool = v0 == 120u32;
        v8
    };
    let v11: bool = if v9 {
        true
    } else {
        let v10: bool = v0 == 85u32;
        v10
    };
    if v11 {
        true
    } else {
        let v12: bool = v0 == 45u32;
        v12
    }
}
fn flags_at_101(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH6>) -> US11 {
    match &*v5 {
        UH6::UH6_Cons(v6, v7) => {
            let v6: u32 = *v6;
            let v7: Rc<UH6> = v7.clone();
            match Rc::unwrap_or_clone(v7) {
                UH6::UH6_Cons(v8, v9) => {
                    let v8: u32 = v8;
                    let mut v9: Rc<UH6> = v9;
                    match Rc::unwrap_or_clone(v9) {
                        UH6::UH6_Cons(v10, _) => {
                            let v10: u32 = v10;
                            let v12: bool = v6 == 40u32;
                            let v14: bool = if v12 {
                                let v13: bool = v8 == 63u32;
                                v13
                            } else {
                                false
                            };
                            let v16: bool = if v14 {
                                is_flag_start_104(v10)
                            } else {
                                false
                            };
                            if v16 {
                                let v17: bool = true;
                                let v18: bool = false;
                                let v19: Rc<UH6> = after_one_44(v5);
                                let v20: Rc<UH6> = after_one_44(v19);
                                scan_flags_102(v0, v1, v2, v3, v4, v17, v18, v20)
                            } else {
                                US11::US11_NotFlags
                            }
                        }
                        _ => {
                            US11::US11_NotFlags
                        }
                    }
                }
                _ => {
                    US11::US11_NotFlags
                }
            }
        }
        _ => {
            US11::US11_NotFlags
        }
    }
}
fn rust_group_105(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US3) -> US3 {
    match &v5 {
        US3::US3_Got(v6, v7, _, _, _, _, _) => {
            let v6: Rc<UH4> = v6.clone();
            let v7: Rc<UH6> = v7.clone();
            let v13: u32 = 41u32;
            let v14: bool = heads_20(v13, v7.clone());
            if v14 {
                let v15: Rc<UH6> = after_one_44(v7);
                US3::US3_Got(v6, v15, v0, v1, v2, v3, v4)
            } else {
                let v17: US2 = US2::US2_GroupUnclosed;
                US3::US3_Stopped(v17)
            }
        }
        _ => {
            v5
        }
    }
}
fn is_quantifier_109(mut v0: u32) -> bool {
    let mut v1: bool = v0 == 63u32;
    let v3: bool = if v1 {
        true
    } else {
        let v2: bool = v0 == 42u32;
        v2
    };
    let mut v5: bool = if v3 {
        true
    } else {
        let v4: bool = v0 == 43u32;
        v4
    };
    if v5 {
        true
    } else {
        let v6: bool = v0 == 123u32;
        v6
    }
}
fn lazy_and_lone_108(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH4>, mut v6: Rc<UH6>) -> US3 {
    let v7: u32 = 63u32;
    let v8: bool = heads_20(v7, v6.clone());
    let v10: Rc<UH6> = if v8 {
        after_one_44(v6)
    } else {
        v6
    };
    let v11: Rc<UH6> = skip_space_94(v0, v1, v2, v3, v4, v10);
    match &*v11 {
        UH6::UH6_Cons(v12, _) => {
            let v12: u32 = *v12;
            let v14: bool = is_quantifier_109(v12);
            if v14 {
                let v15: US2 = US2::US2_RepeatStacked;
                US3::US3_Stopped(v15)
            } else {
                US3::US3_Got(v5, v11, v0, v1, v2, v3, v4)
            }
        }
        UH6::UH6_Nil => {
            let v19: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
            US3::US3_Got(v5, v19, v0, v1, v2, v3, v4)
        }
    }
}
fn bounded_repeat_114(mut v0: Rc<UH4>, mut v1: u32, mut v2: u32, mut v3: US5) -> Rc<UH4> {
    let v4: bool = v2 == 0u32;
    if v4 {
        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
    } else {
        let v6: bool = v2 < v1;
        if v6 {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
        } else {
            let v8: bool = v1 == 1u32;
            let v10: bool = if v8 {
                let mut v9: bool = v2 == 1u32;
                v9
            } else {
                false
            };
            if v10 {
                v0
            } else {
                match &*v0 {
                    UH4::UH4_Epsilon => {
                        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
                    }
                    UH4::UH4_Nothing => {
                        let v11: bool = v1 == 0u32;
                        if v11 {
                            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
                        } else {
                            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
                        }
                    }
                    _ => {
                        Rc::new(UH4::UH4_Repeat(v0, v1, v3))
                    }
                }
            }
        }
    }
}
fn repeat_113(mut v0: Rc<UH4>, mut v1: u32, mut v2: US5) -> Rc<UH4> {
    match &v2 {
        US5::US5_Bounded(v3) => {
            let v3: u32 = *v3;
            bounded_repeat_114(v0, v1, v3, v2)
        }
        US5::US5_Unbounded => {
            let mut v5: bool = v1 == 0u32;
            if v5 {
                star_66(v0)
            } else {
                match &*v0 {
                    UH4::UH4_Epsilon => {
                        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) }
                    }
                    UH4::UH4_Nothing => {
                        { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
                    }
                    _ => {
                        Rc::new(UH4::UH4_Repeat(v0, v1, v2))
                    }
                }
            }
        }
    }
}
fn count_close_112(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH4>, mut v6: u32, mut v7: US5, mut v8: Rc<UH6>) -> US3 {
    let v12: bool = match &v7 {
        US5::US5_Bounded(v9) => {
            let mut v9: u32 = *v9;
            let v10: bool = v6 <= v9;
            v10
        }
        US5::US5_Unbounded => {
            true
        }
    };
    let v13: u32 = 125u32;
    let v14: bool = heads_20(v13, v8.clone());
    let v15: bool = v14 && v12;
    if v15 {
        let v16: Rc<UH4> = repeat_113(v5, v6, v7);
        let v17: Rc<UH6> = after_one_44(v8);
        lazy_and_lone_108(v0, v1, v2, v3, v4, v16, v17)
    } else {
        let v19: US2 = US2::US2_CountInvalid;
        US3::US3_Stopped(v19)
    }
}
fn count_digits_115(mut v0: Rc<UH6>, mut v1: u32, mut v2: bool) -> US13 {
    loop {
        match &*v0 {
            UH6::UH6_Cons(v3, v4) => {
                let v3: u32 = *v3;
                let v4: Rc<UH6> = v4.clone();
                let mut v5: bool = 48u32 <= v3;
                let v7: bool = if v5 {
                    let v6: bool = v3 <= 57u32;
                    v6
                } else {
                    false
                };
                let mut v9: bool = if v7 {
                    let v8: bool = v1 < 100000u32;
                    v8
                } else {
                    false
                };
                if v9 {
                    let v10: u32 = v1.wrapping_mul(10u32);
                    let v11: u32 = v10.wrapping_add(v3);
                    let v12: u32 = v11.wrapping_sub(48u32);
                    let v13: bool = true;
                    (v0, v1, v2) = (v4, v12, v13);
                    continue;
                } else {
                    if v2 {
                        return US13::US13_CountNumber(v1, v0);
                    } else {
                        return US13::US13_NoCountNumber;
                    }
                }
            }
            UH6::UH6_Nil => {
                if v2 {
                    let v19: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                    return US13::US13_CountNumber(v1, v19);
                } else {
                    return US13::US13_NoCountNumber;
                }
            }
        }
    }
}
fn count_upper_111(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH4>, mut v6: u32, mut v7: Rc<UH6>) -> US3 {
    let v8: u32 = 125u32;
    let mut v9: bool = heads_20(v8, v7.clone());
    if v9 {
        let v10: US5 = US5::US5_Bounded(v6);
        count_close_112(v0, v1, v2, v3, v4, v5, v6, v10, v7)
    } else {
        let v12: u32 = 44u32;
        let v13: bool = heads_20(v12, v7.clone());
        if v13 {
            let v14: Rc<UH6> = after_one_44(v7);
            let v15: u32 = 125u32;
            let v16: bool = heads_20(v15, v14.clone());
            if v16 {
                let v17: US5 = US5::US5_Unbounded;
                count_close_112(v0, v1, v2, v3, v4, v5, v6, v17, v14)
            } else {
                let v19: u32 = 0u32;
                let v20: bool = false;
                let v21: US13 = count_digits_115(v14, v19, v20);
                match v21 {
                    US13::US13_CountNumber(v22, v23) => {
                        let v22: u32 = v22;
                        let v23: Rc<UH6> = v23;
                        let v24: US5 = US5::US5_Bounded(v22);
                        count_close_112(v0, v1, v2, v3, v4, v5, v6, v24, v23)
                    }
                    US13::US13_NoCountNumber => {
                        let v26: US2 = US2::US2_CountInvalid;
                        US3::US3_Stopped(v26)
                    }
                }
            }
        } else {
            let v31: US2 = US2::US2_CountInvalid;
            US3::US3_Stopped(v31)
        }
    }
}
fn rust_counted_110(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH4>, mut v6: Rc<UH6>) -> US3 {
    let v7: u32 = 0u32;
    let v8: bool = false;
    let mut v9: US13 = count_digits_115(v6, v7, v8);
    match v9 {
        US13::US13_CountNumber(v10, v11) => {
            let v10: u32 = v10;
            let v11: Rc<UH6> = v11;
            count_upper_111(v0, v1, v2, v3, v4, v5, v10, v11)
        }
        US13::US13_NoCountNumber => {
            let v13: US2 = US2::US2_CountInvalid;
            US3::US3_Stopped(v13)
        }
    }
}
fn rust_quantify_107(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH4>, mut v6: Rc<UH6>) -> US3 {
    let v7: Rc<UH6> = skip_space_94(v0, v1, v2, v3, v4, v6);
    match &*v7 {
        UH6::UH6_Cons(v8, v9) => {
            let v8: u32 = *v8;
            let mut v9: Rc<UH6> = v9.clone();
            let v10: bool = v8 == 63u32;
            if v10 {
                let v11: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
                let v12: Rc<UH4> = alt_28(v11, v5);
                lazy_and_lone_108(v0, v1, v2, v3, v4, v12, v9)
            } else {
                let v14: bool = v8 == 42u32;
                if v14 {
                    let v15: Rc<UH4> = star_66(v5);
                    lazy_and_lone_108(v0, v1, v2, v3, v4, v15, v9)
                } else {
                    let v17: bool = v8 == 43u32;
                    if v17 {
                        let v18: Rc<UH4> = star_66(v5.clone());
                        let v19: Rc<UH4> = seq_67(v5, v18);
                        lazy_and_lone_108(v0, v1, v2, v3, v4, v19, v9)
                    } else {
                        let v21: bool = v8 == 123u32;
                        if v21 {
                            rust_counted_110(v0, v1, v2, v3, v4, v5, v9)
                        } else {
                            US3::US3_Got(v5, v7, v0, v1, v2, v3, v4)
                        }
                    }
                }
            }
        }
        UH6::UH6_Nil => {
            let v28: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
            US3::US3_Got(v5, v28, v0, v1, v2, v3, v4)
        }
    }
}
fn rust_quantified_106(mut v0: US3) -> US3 {
    match &v0 {
        US3::US3_Got(v1, v2, v3, v4, v5, v6, v7) => {
            let mut v1: Rc<UH4> = v1.clone();
            let v2: Rc<UH6> = v2.clone();
            let v3: bool = *v3;
            let v4: bool = *v4;
            let mut v5: bool = *v5;
            let v6: bool = *v6;
            let v7: bool = *v7;
            rust_quantify_107(v3, v4, v5, v6, v7, v1, v2)
        }
        _ => {
            v0
        }
    }
}
fn is_ascii_alnum_120(mut v0: u32) -> bool {
    let mut v1: bool = 48u32 <= v0;
    let v3: bool = if v1 {
        let v2: bool = v0 <= 57u32;
        v2
    } else {
        false
    };
    let v7: bool = if v3 {
        true
    } else {
        let v4: bool = 65u32 <= v0;
        if v4 {
            let mut v5: bool = v0 <= 90u32;
            v5
        } else {
            false
        }
    };
    if v7 {
        true
    } else {
        let v8: bool = 97u32 <= v0;
        if v8 {
            let mut v9: bool = v0 <= 122u32;
            v9
        } else {
            false
        }
    }
}
fn skip_name_119(mut v0: Rc<UH6>) -> US14 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v1, v2) => {
                let mut v1: u32 = v1;
                let v2: Rc<UH6> = v2;
                let v3: bool = v1 == 62u32;
                if v3 {
                    return US14::US14_Some(v2);
                } else {
                    let mut v5: bool = is_ascii_alnum_120(v1);
                    let v7: bool = if v5 {
                        true
                    } else {
                        let v6: bool = v1 == 95u32;
                        v6
                    };
                    let mut v9: bool = if v7 {
                        true
                    } else {
                        let v8: bool = v1 == 46u32;
                        v8
                    };
                    let v11: bool = if v9 {
                        true
                    } else {
                        let v10: bool = v1 == 91u32;
                        v10
                    };
                    let v13: bool = if v11 {
                        true
                    } else {
                        let v12: bool = v1 == 93u32;
                        v12
                    };
                    if v13 {
                        v0 = v2;
                        continue;
                    } else {
                        return US14::US14_None;
                    }
                }
            }
            UH6::UH6_Nil => {
                return US14::US14_None;
            }
        }
    }
}
fn rust_named_118(mut v0: Rc<UH6>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool) -> US3 {
    let v6: US14 = skip_name_119(v0);
    match v6 {
        US14::US14_None => {
            let v11: US2 = US2::US2_GroupUnclosed;
            US3::US3_Stopped(v11)
        }
        US14::US14_Some(v7) => {
            let v7: Rc<UH6> = v7;
            let v8: bool = false;
            let mut v9: US3 = rust_alternation_25(v7, v1, v2, v3, v4, v5, v8);
            rust_group_105(v1, v2, v3, v4, v5, v9)
        }
    }
}
fn rust_angle_121(mut v0: Rc<UH6>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool) -> US3 {
    let v6: u32 = 61u32;
    let v7: bool = heads_20(v6, v0.clone());
    let v10: bool = if v7 {
        true
    } else {
        let v8: u32 = 33u32;
        heads_20(v8, v0.clone())
    };
    if v10 {
        let v11: US2 = US2::US2_LookaroundUnsupported;
        US3::US3_Stopped(v11)
    } else {
        rust_named_118(v0, v1, v2, v3, v4, v5)
    }
}
fn rust_open_117(mut v0: Rc<UH6>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool) -> US3 {
    match &*v0 {
        UH6::UH6_Cons(v6, v7) => {
            let v6: u32 = *v6;
            let v7: Rc<UH6> = v7.clone();
            let v8: bool = v6 != 63u32;
            if v8 {
                let mut v9: bool = false;
                let v10: US3 = rust_alternation_25(v0, v1, v2, v3, v4, v5, v9);
                rust_group_105(v1, v2, v3, v4, v5, v10)
            } else {
                let v12: u32 = 58u32;
                let v13: bool = heads_20(v12, v7.clone());
                if v13 {
                    let v14: Rc<UH6> = after_one_44(v7);
                    let v15: bool = false;
                    let v16: US3 = rust_alternation_25(v14, v1, v2, v3, v4, v5, v15);
                    rust_group_105(v1, v2, v3, v4, v5, v16)
                } else {
                    let v18: u32 = 61u32;
                    let v19: bool = heads_20(v18, v7.clone());
                    let v22: bool = if v19 {
                        true
                    } else {
                        let v20: u32 = 33u32;
                        heads_20(v20, v7.clone())
                    };
                    if v22 {
                        let v23: US2 = US2::US2_LookaroundUnsupported;
                        US3::US3_Stopped(v23)
                    } else {
                        let v25: u32 = 80u32;
                        let v26: bool = heads_20(v25, v7.clone());
                        let v30: bool = if v26 {
                            let v27: u32 = 60u32;
                            let v28: Rc<UH6> = after_one_44(v7.clone());
                            heads_20(v27, v28)
                        } else {
                            false
                        };
                        if v30 {
                            let v31: Rc<UH6> = after_one_44(v7);
                            let v32: Rc<UH6> = after_one_44(v31);
                            rust_named_118(v32, v1, v2, v3, v4, v5)
                        } else {
                            let v34: u32 = 80u32;
                            let v35: bool = heads_20(v34, v7.clone());
                            let v39: bool = if v35 {
                                let v36: u32 = 61u32;
                                let v37: Rc<UH6> = after_one_44(v7.clone());
                                heads_20(v36, v37)
                            } else {
                                false
                            };
                            if v39 {
                                let v40: US2 = US2::US2_BackreferenceUnsupported;
                                US3::US3_Stopped(v40)
                            } else {
                                let v42: u32 = 60u32;
                                let v43: bool = heads_20(v42, v7.clone());
                                if v43 {
                                    let v44: Rc<UH6> = after_one_44(v7);
                                    rust_angle_121(v44, v1, v2, v3, v4, v5)
                                } else {
                                    let v46: US2 = US2::US2_FlagUnknown;
                                    US3::US3_Stopped(v46)
                                }
                            }
                        }
                    }
                }
            }
        }
        UH6::UH6_Nil => {
            let v54: US2 = US2::US2_GroupUnclosed;
            US3::US3_Stopped(v54)
        }
    }
}
fn class_to_step_122(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US15) -> US3 {
    match v5 {
        US15::US15_ClassGot(v6, v7) => {
            let v6: US4 = v6;
            let v7: Rc<UH6> = v7;
            let v8: bool = is_empty_61(v6.clone());
            let v11: Rc<UH4> = if v8 {
                { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
            } else {
                Rc::new(UH4::UH4_Set(v6))
            };
            US3::US3_Got(v11, v7, v0, v1, v2, v3, v4)
        }
        US15::US15_ClassStopped(v13) => {
            let v13: US2 = v13;
            US3::US3_Stopped(v13)
        }
    }
}
fn beyond_ascii_127() -> US4 {
    let mut v0: u32 = 128u32;
    let mut v1: u32 = 1114111u32;
    let v2: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH5> = Rc::new(UH5::UH5_Cons(v0, v1, v2));
    let v4: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH5> = sorted_by_low_82(v3, v4);
    let v6: Rc<UH5> = merged_84(v5);
    US4::US4_Normal(v6)
}
fn method130(mut v0: Rc<UH5>, mut v1: Rc<UH5>) -> Rc<UH5> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v2, v3, v4) => {
            let v2: u32 = v2;
            let v3: u32 = v3;
            let v4: Rc<UH5> = v4;
            let mut v5: Rc<UH5> = method130(v4, v1);
            Rc::new(UH5::UH5_Cons(v2, v3, v5))
        }
        UH5::UH5_Nil => {
            v1
        }
    }
}
fn set_union_129(mut v0: US4, mut v1: US4) -> US4 {
    let v3: Rc<UH5> = match v0 {
        US4::US4_Normal(v2) => {
            let v2: Rc<UH5> = v2;
            v2
        }
    };
    let mut v5: Rc<UH5> = match v1 {
        US4::US4_Normal(v4) => {
            let v4: Rc<UH5> = v4;
            v4
        }
    };
    let v6: Rc<UH5> = method130(v3, v5);
    let v7: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH5> = sorted_by_low_82(v6, v7);
    let mut v9: Rc<UH5> = merged_84(v8);
    US4::US4_Normal(v9)
}
fn intersection_128(mut v0: US4, mut v1: US4) -> US4 {
    let v2: US4 = complement_62(v0);
    let v3: US4 = complement_62(v1);
    let v4: US4 = set_union_129(v2, v3);
    complement_62(v4)
}
fn method133(mut v0: bool, mut v1: Rc<UH5>, mut v2: Rc<UH5>) -> Rc<UH5> {
    match Rc::unwrap_or_clone(v1) {
        UH5::UH5_Cons(v3, v4, v5) => {
            let v3: u32 = v3;
            let v4: u32 = v4;
            let mut v5: Rc<UH5> = v5;
            let v6: Rc<UH5> = method133(v0, v5, v2);
            let (mut v11, mut v12): (u32, u32) = if v0 {
                let v7: u32 = v3.wrapping_sub(32u32);
                let v8: u32 = v4.wrapping_sub(32u32);
                (v7, v8)
            } else {
                let mut v9: u32 = v3.wrapping_add(32u32);
                let v10: u32 = v4.wrapping_add(32u32);
                (v9, v10)
            };
            Rc::new(UH5::UH5_Cons(v11, v12, v6))
        }
        UH5::UH5_Nil => {
            v2
        }
    }
}
fn shifted_132(mut v0: US4, mut v1: bool) -> US4 {
    let v3: Rc<UH5> = match v0 {
        US4::US4_Normal(v2) => {
            let v2: Rc<UH5> = v2;
            v2
        }
    };
    let v4: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH5> = method133(v1, v3, v4);
    let v6: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH5> = sorted_by_low_82(v5, v6);
    let v8: Rc<UH5> = merged_84(v7);
    US4::US4_Normal(v8)
}
fn ascii_upper_134() -> US4 {
    let mut v0: u32 = 65u32;
    let mut v1: u32 = 90u32;
    let v2: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH5> = Rc::new(UH5::UH5_Cons(v0, v1, v2));
    let v4: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH5> = sorted_by_low_82(v3, v4);
    let v6: Rc<UH5> = merged_84(v5);
    US4::US4_Normal(v6)
}
fn contains_in_136(mut v0: u32, mut v1: Rc<UH5>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH5::UH5_Cons(v2, v3, v4) => {
                let v2: u32 = v2;
                let v3: u32 = v3;
                let v4: Rc<UH5> = v4;
                let mut v5: bool = v0 < v2;
                if v5 {
                    return false;
                } else {
                    let v6: bool = v0 <= v3;
                    if v6 {
                        return true;
                    } else {
                        v1 = v4;
                        continue;
                    }
                }
            }
            UH5::UH5_Nil => {
                return false;
            }
        }
    }
}
fn contains_135(mut v0: u32, mut v1: US4) -> bool {
    let v3: Rc<UH5> = match v1 {
        US4::US4_Normal(v2) => {
            let v2: Rc<UH5> = v2;
            v2
        }
    };
    contains_in_136(v0, v3)
}
fn nothing_set_137() -> US4 {
    let mut v0: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v1: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH5> = sorted_by_low_82(v0, v1);
    let v3: Rc<UH5> = merged_84(v2);
    US4::US4_Normal(v3)
}
fn folded_131(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US4) -> US4 {
    let v6: u32 = 97u32;
    let v7: u32 = 122u32;
    let v8: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH5> = Rc::new(UH5::UH5_Cons(v6, v7, v8));
    let v10: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH5> = sorted_by_low_82(v9, v10);
    let v12: Rc<UH5> = merged_84(v11);
    let v13: US4 = US4::US4_Normal(v12);
    let v14: US4 = intersection_128(v5.clone(), v13);
    let v15: bool = true;
    let v16: US4 = shifted_132(v14, v15);
    let v17: US4 = ascii_upper_134();
    let v18: US4 = intersection_128(v5.clone(), v17);
    let v19: bool = false;
    let v20: US4 = shifted_132(v18, v19);
    let v21: US4 = set_union_129(v16, v20);
    let v22: US4 = set_union_129(v5, v21);
    let v25: bool = if v3 {
        let v23: u32 = 107u32;
        contains_135(v23, v22.clone())
    } else {
        false
    };
    let v30: US4 = if v25 {
        let v26: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
        let v27: Rc<UH5> = Rc::new(UH5::UH5_Cons(8490u32, 8490u32, v26));
        US4::US4_Normal(v27)
    } else {
        nothing_set_137()
    };
    let v33: bool = if v3 {
        let v31: u32 = 115u32;
        contains_135(v31, v22.clone())
    } else {
        false
    };
    let v38: US4 = if v33 {
        let v34: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
        let v35: Rc<UH5> = Rc::new(UH5::UH5_Cons(383u32, 383u32, v34));
        US4::US4_Normal(v35)
    } else {
        nothing_set_137()
    };
    let v39: US4 = set_union_129(v30, v38);
    set_union_129(v22, v39)
}
fn fold_class_126(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US4, mut v6: Rc<UH6>) -> US15 {
    let v7: bool = v0 == false;
    if v7 {
        US15::US15_ClassGot(v5, v6)
    } else {
        let mut v9: US4 = beyond_ascii_127();
        let v10: US4 = intersection_128(v5.clone(), v9);
        let v11: bool = is_empty_61(v10);
        if v11 {
            let v12: US4 = folded_131(v0, v1, v2, v3, v4, v5);
            US15::US15_ClassGot(v12, v6)
        } else {
            let v14: US2 = US2::US2_CaseFoldBeyondAscii;
            US15::US15_ClassStopped(v14)
        }
    }
}
fn literal_class_125(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: u32, mut v6: Rc<UH6>) -> US15 {
    let v7: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH5> = Rc::new(UH5::UH5_Cons(v5, v5, v7));
    let mut v9: US4 = US4::US4_Normal(v8);
    fold_class_126(v0, v1, v2, v3, v4, v9, v6)
}
fn literal_step_124(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: u32, mut v6: Rc<UH6>) -> US3 {
    let v7: US15 = literal_class_125(v0, v1, v2, v3, v4, v5, v6);
    class_to_step_122(v0, v1, v2, v3, v4, v7)
}
fn escape_step_123(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US16) -> US3 {
    match v5 {
        US16::US16_EscChar(v6, v7) => {
            let v6: u32 = v6;
            let v7: Rc<UH6> = v7;
            literal_step_124(v0, v1, v2, v3, v4, v6, v7)
        }
        US16::US16_EscSet(v9, v10) => {
            let mut v9: US4 = v9;
            let v10: Rc<UH6> = v10;
            let v11: bool = is_empty_61(v9.clone());
            let v14: Rc<UH4> = if v11 {
                { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
            } else {
                Rc::new(UH4::UH4_Set(v9))
            };
            US3::US3_Got(v14, v10, v0, v1, v2, v3, v4)
        }
        US16::US16_EscStopped(v16) => {
            let v16: US2 = v16;
            US3::US3_Stopped(v16)
        }
    }
}
fn class_skip_141(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH6>) -> Rc<UH6> {
    loop {
        match &*v5 {
            UH6::UH6_Cons(v6, v7) => {
                let v6: u32 = *v6;
                let v7: Rc<UH6> = v7.clone();
                let mut v9: bool = if v4 {
                    is_space_code_96(v6)
                } else {
                    false
                };
                if v9 {
                    v5 = v7;
                    continue;
                } else {
                    return v5;
                }
            }
            UH6::UH6_Nil => {
                return { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn starts_op_142(mut v0: &Rc<UH6>) -> bool {
    match &**v0 {
        UH6::UH6_Cons(v1, v2) => {
            let mut v1: u32 = *v1;
            let v2: Rc<UH6> = v2.clone();
            match Rc::unwrap_or_clone(v2) {
                UH6::UH6_Cons(v3, _) => {
                    let v3: u32 = v3;
                    let mut v5: bool = v1 == 38u32;
                    let v7: bool = if v5 {
                        let v6: bool = v3 == 38u32;
                        v6
                    } else {
                        false
                    };
                    let v11: bool = if v7 {
                        true
                    } else {
                        let v8: bool = v1 == 45u32;
                        if v8 {
                            let mut v9: bool = v3 == 45u32;
                            v9
                        } else {
                            false
                        }
                    };
                    if v11 {
                        true
                    } else {
                        let v12: bool = v1 == 126u32;
                        if v12 {
                            let v13: bool = v3 == 126u32;
                            v13
                        } else {
                            false
                        }
                    }
                }
                _ => {
                    false
                }
            }
        }
        _ => {
            false
        }
    }
}
fn method147(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> Rc<UH6> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v2, v3) => {
                let v2: u32 = v2;
                let v3: Rc<UH6> = v3;
                let v4: Rc<UH6> = Rc::new(UH6::UH6_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH6::UH6_Nil => {
                return v1;
            }
        }
    }
}
fn posix_name_146(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> US18 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v2, v3) => {
                let v2: u32 = v2;
                let v3: Rc<UH6> = v3;
                let v4: bool = v2 == 58u32;
                let v7: bool = if v4 {
                    let mut v5: u32 = 93u32;
                    heads_20(v5, v3.clone())
                } else {
                    false
                };
                if v7 {
                    let v8: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                    let mut v9: Rc<UH6> = method147(v1, v8);
                    let v10: Rc<UH6> = after_one_44(v3);
                    return US18::US18_Some(v9, v10);
                } else {
                    let v12: bool = 97u32 <= v2;
                    let v14: bool = if v12 {
                        let v13: bool = v2 <= 122u32;
                        v13
                    } else {
                        false
                    };
                    if v14 {
                        let v15: Rc<UH6> = Rc::new(UH6::UH6_Cons(v2, v1));
                        (v0, v1) = (v3, v15);
                        continue;
                    } else {
                        return US18::US18_None;
                    }
                }
            }
            UH6::UH6_Nil => {
                return US18::US18_None;
            }
        }
    }
}
fn posix_names_148() -> Rc<UH11> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alnum"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH5> = Rc::new(UH5::UH5_Cons(97u32, 122u32, v1));
    let v3: Rc<UH5> = Rc::new(UH5::UH5_Cons(65u32, 90u32, v2));
    let v4: Rc<UH5> = Rc::new(UH5::UH5_Cons(48u32, 57u32, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alpha"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH5> = Rc::new(UH5::UH5_Cons(97u32, 122u32, v6));
    let v8: Rc<UH5> = Rc::new(UH5::UH5_Cons(65u32, 90u32, v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ascii"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH5> = Rc::new(UH5::UH5_Cons(0u32, 127u32, v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blank"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH5> = Rc::new(UH5::UH5_Cons(32u32, 32u32, v13));
    let v15: Rc<UH5> = Rc::new(UH5::UH5_Cons(9u32, 9u32, v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cntrl"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH5> = Rc::new(UH5::UH5_Cons(127u32, 127u32, v17));
    let v19: Rc<UH5> = Rc::new(UH5::UH5_Cons(0u32, 31u32, v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("digit"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v22: Rc<UH5> = Rc::new(UH5::UH5_Cons(48u32, 57u32, v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("graph"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v25: Rc<UH5> = Rc::new(UH5::UH5_Cons(33u32, 126u32, v24));
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lower"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v28: Rc<UH5> = Rc::new(UH5::UH5_Cons(97u32, 122u32, v27));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("print"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v31: Rc<UH5> = Rc::new(UH5::UH5_Cons(32u32, 126u32, v30));
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("punct"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v34: Rc<UH5> = Rc::new(UH5::UH5_Cons(123u32, 126u32, v33));
    let v35: Rc<UH5> = Rc::new(UH5::UH5_Cons(91u32, 96u32, v34));
    let v36: Rc<UH5> = Rc::new(UH5::UH5_Cons(58u32, 64u32, v35));
    let v37: Rc<UH5> = Rc::new(UH5::UH5_Cons(33u32, 47u32, v36));
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("space"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v40: Rc<UH5> = Rc::new(UH5::UH5_Cons(32u32, 32u32, v39));
    let v41: Rc<UH5> = Rc::new(UH5::UH5_Cons(9u32, 13u32, v40));
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("upper"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v44: Rc<UH5> = Rc::new(UH5::UH5_Cons(65u32, 90u32, v43));
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("word"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v47: Rc<UH5> = Rc::new(UH5::UH5_Cons(95u32, 95u32, v46));
    let v48: Rc<UH5> = Rc::new(UH5::UH5_Cons(97u32, 122u32, v47));
    let v49: Rc<UH5> = Rc::new(UH5::UH5_Cons(65u32, 90u32, v48));
    let v50: Rc<UH5> = Rc::new(UH5::UH5_Cons(48u32, 57u32, v49));
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("xdigit"); } LIT.with(|lit| lit.clone()) };
    let v52: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v53: Rc<UH5> = Rc::new(UH5::UH5_Cons(97u32, 102u32, v52));
    let v54: Rc<UH5> = Rc::new(UH5::UH5_Cons(65u32, 70u32, v53));
    let v55: Rc<UH5> = Rc::new(UH5::UH5_Cons(48u32, 57u32, v54));
    let v56: Rc<UH11> = { thread_local!{ static CASE: Rc<UH11> = Rc::new(UH11::UH11_Nil); } CASE.with(|case| case.clone()) };
    let v57: Rc<UH11> = Rc::new(UH11::UH11_Cons(v51, v55, v56));
    let v58: Rc<UH11> = Rc::new(UH11::UH11_Cons(v45, v50, v57));
    let v59: Rc<UH11> = Rc::new(UH11::UH11_Cons(v42, v44, v58));
    let v60: Rc<UH11> = Rc::new(UH11::UH11_Cons(v38, v41, v59));
    let v61: Rc<UH11> = Rc::new(UH11::UH11_Cons(v32, v37, v60));
    let v62: Rc<UH11> = Rc::new(UH11::UH11_Cons(v29, v31, v61));
    let v63: Rc<UH11> = Rc::new(UH11::UH11_Cons(v26, v28, v62));
    let v64: Rc<UH11> = Rc::new(UH11::UH11_Cons(v23, v25, v63));
    let v65: Rc<UH11> = Rc::new(UH11::UH11_Cons(v20, v22, v64));
    let v66: Rc<UH11> = Rc::new(UH11::UH11_Cons(v16, v19, v65));
    let v67: Rc<UH11> = Rc::new(UH11::UH11_Cons(v12, v15, v66));
    let v68: Rc<UH11> = Rc::new(UH11::UH11_Cons(v9, v11, v67));
    let v69: Rc<UH11> = Rc::new(UH11::UH11_Cons(v5, v8, v68));
    Rc::new(UH11::UH11_Cons(v0, v4, v69))
}
fn method150(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> bool {
    loop {
        match (Rc::unwrap_or_clone(v1), Rc::unwrap_or_clone(v0)) {
            (UH6::UH6_Cons(v2, v3), UH6::UH6_Cons(v4, v5)) => {
                let v2: u32 = v2;
                let v3: Rc<UH6> = v3;
                let v4: u32 = v4;
                let mut v5: Rc<UH6> = v5;
                let v6: bool = v2 == v4;
                if v6 {
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return false;
                }
            }
            (UH6::UH6_Nil, UH6::UH6_Nil) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
fn posix_lookup_149(mut v0: Rc<UH6>, mut v1: Rc<UH11>) -> US19 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH11::UH11_Cons(v2, v3, v4) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH5> = v3;
                let v4: Rc<UH11> = v4;
                let mut v5: Rc<UH6> = rust_codes_21(v2);
                let v6: bool = method150(v0.clone(), v5);
                if v6 {
                    let v7: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
                    let v8: Rc<UH5> = sorted_by_low_82(v3, v7);
                    let mut v9: Rc<UH5> = merged_84(v8);
                    let v10: US4 = US4::US4_Normal(v9);
                    return US19::US19_Some(v10);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH11::UH11_Nil => {
                return US19::US19_None;
            }
        }
    }
}
fn posix_class_145(mut v0: Rc<UH6>) -> US17 {
    let mut v1: u32 = 94u32;
    let v2: bool = heads_20(v1, v0.clone());
    let v4: Rc<UH6> = if v2 {
        after_one_44(v0)
    } else {
        v0
    };
    let mut v5: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    let v6: US18 = posix_name_146(v4, v5);
    match v6 {
        US18::US18_None => {
            US17::US17_None
        }
        US18::US18_Some(v7, v8) => {
            let v7: Rc<UH6> = v7;
            let v8: Rc<UH6> = v8;
            let mut v9: Rc<UH11> = posix_names_148();
            let v10: US19 = posix_lookup_149(v7, v9);
            match v10 {
                US19::US19_None => {
                    let v16: US2 = US2::US2_EscapeUnknown;
                    let v17: US15 = US15::US15_ClassStopped(v16);
                    US17::US17_Some(v17)
                }
                US19::US19_Some(v11) => {
                    let v11: US4 = v11;
                    let v13: US4 = if v2 {
                        complement_62(v11)
                    } else {
                        v11
                    };
                    let v14: US15 = US15::US15_ClassGot(v13, v8);
                    US17::US17_Some(v14)
                }
            }
        }
    }
}
fn class_nested_144(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH6>) -> US15 {
    let v6: u32 = 58u32;
    let v7: bool = heads_20(v6, v5.clone());
    if v7 {
        let v8: Rc<UH6> = after_one_44(v5.clone());
        let mut v9: US17 = posix_class_145(v8);
        match v9 {
            US17::US17_None => {
                class_open_138(v0, v1, v2, v3, v4, v5)
            }
            US17::US17_Some(v10) => {
                let v10: US15 = v10;
                match &v10 {
                    US15::US15_ClassGot(v11, v12) => {
                        let v11: US4 = v11.clone();
                        let v12: Rc<UH6> = v12.clone();
                        fold_class_126(v0, v1, v2, v3, v4, v11, v12)
                    }
                    _ => {
                        v10
                    }
                }
            }
        }
    } else {
        class_open_138(v0, v1, v2, v3, v4, v5)
    }
}
fn perl_set_156(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US4, mut v6: US4, mut v7: bool, mut v8: Rc<UH6>) -> US16 {
    let mut v9: US4 = if v3 {
        v6
    } else {
        v5
    };
    let v11: US4 = if v7 {
        complement_62(v9)
    } else {
        v9
    };
    US16::US16_EscSet(v11, v8)
}
fn unicode_space_158() -> US4 {
    let mut v0: u32 = 9u32;
    let mut v1: u32 = 13u32;
    let v2: u32 = 32u32;
    let v3: u32 = 32u32;
    let v4: u32 = 133u32;
    let mut v5: u32 = 133u32;
    let v6: u32 = 160u32;
    let v7: u32 = 160u32;
    let v8: u32 = 5760u32;
    let mut v9: u32 = 5760u32;
    let v10: u32 = 8192u32;
    let v11: u32 = 8202u32;
    let v12: u32 = 8232u32;
    let v13: u32 = 8233u32;
    let v14: u32 = 8239u32;
    let v15: u32 = 8239u32;
    let v16: u32 = 8287u32;
    let v17: u32 = 8287u32;
    let v18: u32 = 12288u32;
    let v19: u32 = 12288u32;
    let v20: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH5> = Rc::new(UH5::UH5_Cons(v18, v19, v20));
    let v22: Rc<UH5> = Rc::new(UH5::UH5_Cons(v16, v17, v21));
    let v23: Rc<UH5> = Rc::new(UH5::UH5_Cons(v14, v15, v22));
    let v24: Rc<UH5> = Rc::new(UH5::UH5_Cons(v12, v13, v23));
    let v25: Rc<UH5> = Rc::new(UH5::UH5_Cons(v10, v11, v24));
    let v26: Rc<UH5> = Rc::new(UH5::UH5_Cons(v8, v9, v25));
    let v27: Rc<UH5> = Rc::new(UH5::UH5_Cons(v6, v7, v26));
    let v28: Rc<UH5> = Rc::new(UH5::UH5_Cons(v4, v5, v27));
    let v29: Rc<UH5> = Rc::new(UH5::UH5_Cons(v2, v3, v28));
    let v30: Rc<UH5> = Rc::new(UH5::UH5_Cons(v0, v1, v29));
    let v31: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v32: Rc<UH5> = sorted_by_low_82(v30, v31);
    let v33: Rc<UH5> = merged_84(v32);
    US4::US4_Normal(v33)
}
fn ascii_space_159() -> US4 {
    let mut v0: u32 = 9u32;
    let mut v1: u32 = 13u32;
    let v2: u32 = 32u32;
    let v3: u32 = 32u32;
    let v4: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH5> = Rc::new(UH5::UH5_Cons(v2, v3, v4));
    let v6: Rc<UH5> = Rc::new(UH5::UH5_Cons(v0, v1, v5));
    let v7: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH5> = sorted_by_low_82(v6, v7);
    let mut v9: Rc<UH5> = merged_84(v8);
    US4::US4_Normal(v9)
}
fn space_set_157(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: Rc<UH6>) -> US16 {
    let mut v9: US4 = if v3 {
        unicode_space_158()
    } else {
        ascii_space_159()
    };
    let v11: US4 = if v5 {
        complement_62(v9)
    } else {
        v9
    };
    US16::US16_EscSet(v11, v6)
}
fn split_at_equals_164(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> US18 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v2, v3) => {
                let v2: u32 = v2;
                let v3: Rc<UH6> = v3;
                let v4: bool = v2 == 61u32;
                let v6: bool = if v4 {
                    true
                } else {
                    let mut v5: bool = v2 == 58u32;
                    v5
                };
                if v6 {
                    let v7: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                    let v8: Rc<UH6> = method147(v1, v7);
                    return US18::US18_Some(v8, v3);
                } else {
                    let v10: Rc<UH6> = Rc::new(UH6::UH6_Cons(v2, v1));
                    (v0, v1) = (v3, v10);
                    continue;
                }
            }
            UH6::UH6_Nil => {
                return US18::US18_None;
            }
        }
    }
}
fn key_from_166(mut v0: Rc<UH7>, mut v1: Rc<UH6>) -> Rc<UH6> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH7::UH7_Cons(v2, v3) => {
                let v2: i32 = v2;
                let v3: Rc<UH7> = v3;
                let v4: u32 = (v2 as u32);
                let mut v5: bool = v4 == 32u32;
                let v7: bool = if v5 {
                    true
                } else {
                    let v6: bool = v4 == 95u32;
                    v6
                };
                let mut v9: bool = if v7 {
                    true
                } else {
                    let v8: bool = v4 == 45u32;
                    v8
                };
                if v9 {
                    v0 = v3;
                    continue;
                } else {
                    let v11: bool = 65u32 <= v4;
                    let v13: bool = if v11 {
                        let v12: bool = v4 <= 90u32;
                        v12
                    } else {
                        false
                    };
                    if v13 {
                        let v14: u32 = v4.wrapping_add(32u32);
                        let v15: Rc<UH6> = Rc::new(UH6::UH6_Cons(v14, v1));
                        (v0, v1) = (v3, v15);
                        continue;
                    } else {
                        let v17: Rc<UH6> = Rc::new(UH6::UH6_Cons(v4, v1));
                        (v0, v1) = (v3, v17);
                        continue;
                    }
                }
            }
            UH7::UH7_Nil => {
                let v21: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                return method147(v1, v21);
            }
        }
    }
}
fn method167(mut v0: Rc<UH6>, mut v1: Rc<UH7>) -> Rc<UH7> {
    match Rc::unwrap_or_clone(v0) {
        UH6::UH6_Cons(v2, v3) => {
            let v2: u32 = v2;
            let v3: Rc<UH6> = v3;
            let v4: Rc<UH7> = method167(v3, v1);
            let mut v5: i32 = (v2 as i32);
            Rc::new(UH7::UH7_Cons(v5, v4))
        }
        UH6::UH6_Nil => {
            v1
        }
    }
}
fn loose_key_165(mut v0: Rc<UH6>) -> Rc<UH6> {
    let mut v1: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH7> = method167(v0, v1);
    let v3: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    key_from_166(v2, v3)
}
fn key_of_172(mut v0: Rc<str>) -> Rc<UH6> {
    let mut v1: Rc<Vec<char>> = Rc::new((&*v0).chars().collect());
    let v2: Rc<Vec<char>> = Rc::new((v1).as_ref().iter().rev().cloned().collect::<Vec<_>>());
    let v3: usize = (v2).len();
    let v4: i32 = ((v3) as i32);
    let mut v5: i32 = 0i32;
    let v6: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH7> = method23(v2, v4, v5, v6);
    let v8: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    key_from_166(v7, v8)
}
fn method171(mut v0: Rc<UH6>, mut v1: Rc<UH0>, mut v2: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v7: bool = if v2 {
                    true
                } else {
                    let mut v5: Rc<UH6> = key_of_172(v3);
                    method150(v5, v0.clone())
                };
                (v1, v2) = (v4, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn named_170(mut v0: Rc<UH6>, mut v1: Rc<UH0>) -> bool {
    let v2: bool = false;
    method171(v0, v1, v2)
}
fn script_table_169(mut v0: Rc<UH6>) -> US21 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("latin"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("latn"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v4));
    let v6: bool = named_170(v0.clone(), v5);
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("65,97,170,186,192,216,248,736,7424,7468,7522,7531,7545,7680,8305,8319,8336,8490,8498,8526,8544,11360,42786,42891,42960,42963,42965,42994,43824,43868,43878,64256,65313,65345,67456,67463,67506,122624,122661"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("90,122,170,186,214,246,696,740,7461,7516,7525,7543,7614,7935,8305,8319,8348,8491,8498,8526,8584,11391,42887,42957,42961,42963,42972,43007,43866,43876,43881,64262,65338,65370,67461,67504,67514,122654,122666"); } LIT.with(|lit| lit.clone()) };
        US21::US21_Some(v7, v8)
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("greek"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grek"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v12));
        let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v13));
        let v15: bool = named_170(v0.clone(), v14);
        if v15 {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("880,885,890,895,900,902,904,908,910,931,1008,7462,7517,7526,7615,7936,7960,7968,8008,8016,8025,8027,8029,8031,8064,8118,8134,8150,8157,8178,8182,8486,43877,65856,65952,119296"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("883,887,893,895,900,902,906,908,929,993,1023,7466,7521,7530,7615,7957,7965,8005,8013,8023,8025,8027,8029,8061,8116,8132,8147,8155,8175,8180,8190,8486,43877,65934,65952,119365"); } LIT.with(|lit| lit.clone()) };
            US21::US21_Some(v16, v17)
        } else {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cyrillic"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cyrl"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v21));
            let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v22));
            let v24: bool = named_170(v0.clone(), v23);
            if v24 {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1024,1159,7296,7467,7544,11744,42560,65070,122928,123023"); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1156,1327,7306,7467,7544,11775,42655,65071,122989,123023"); } LIT.with(|lit| lit.clone()) };
                US21::US21_Some(v25, v26)
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("han"); } LIT.with(|lit| lit.clone()) };
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hani"); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v30));
                let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28, v31));
                let v33: bool = named_170(v0.clone(), v32);
                if v33 {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("11904,11931,12032,12293,12295,12321,12344,13312,19968,63744,64112,94178,94192,131072,173824,177984,178208,183984,191472,194560,196608,201552"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("11929,12019,12245,12293,12295,12329,12347,19903,40959,64109,64217,94179,94193,173791,177977,178205,183969,191456,192093,195101,201546,205743"); } LIT.with(|lit| lit.clone()) };
                    US21::US21_Some(v34, v35)
                } else {
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arabic"); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arab"); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v40: Rc<UH0> = Rc::new(UH0::UH0_Cons(v38, v39));
                    let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v40));
                    let v42: bool = named_170(v0.clone(), v41);
                    if v42 {
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1536,1542,1549,1564,1568,1601,1622,1649,1758,1872,2160,2192,2199,2275,64336,64467,64832,64914,64975,65008,65136,65142,69216,69314,69372,126464,126469,126497,126500,126503,126505,126516,126521,126523,126530,126535,126537,126539,126541,126545,126548,126551,126553,126555,126557,126559,126561,126564,126567,126572,126580,126585,126590,126592,126603,126625,126629,126635,126704"); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1540,1547,1562,1566,1599,1610,1647,1756,1791,1919,2190,2193,2273,2303,64450,64829,64911,64967,64975,65023,65140,65276,69246,69316,69375,126467,126495,126498,126500,126503,126514,126519,126521,126523,126530,126535,126537,126539,126543,126546,126548,126551,126553,126555,126557,126559,126562,126564,126570,126578,126583,126588,126590,126601,126619,126627,126633,126651,126705"); } LIT.with(|lit| lit.clone()) };
                        US21::US21_Some(v43, v44)
                    } else {
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hebrew"); } LIT.with(|lit| lit.clone()) };
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hebr"); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v47, v48));
                        let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v46, v49));
                        let v51: bool = named_170(v0, v50);
                        if v51 {
                            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1425,1488,1519,64285,64312,64318,64320,64323,64326"); } LIT.with(|lit| lit.clone()) };
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1479,1514,1524,64310,64316,64318,64321,64324,64335"); } LIT.with(|lit| lit.clone()) };
                            US21::US21_Some(v52, v53)
                        } else {
                            US21::US21_None
                        }
                    }
                }
            }
        }
    }
}
fn category_table_173(mut v0: Rc<UH6>) -> US21 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("l"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("letter"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v4));
    let v6: bool = named_170(v0.clone(), v5);
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("65,97,170,181,186,192,216,248,710,736,748,750,880,886,890,895,902,904,908,910,931,1015,1162,1329,1369,1376,1488,1519,1568,1646,1649,1749,1765,1774,1786,1791,1808,1810,1869,1969,1994,2036,2042,2048,2074,2084,2088,2112,2144,2160,2185,2208,2308,2365,2384,2392,2417,2437,2447,2451,2474,2482,2486,2493,2510,2524,2527,2544,2556,2565,2575,2579,2602,2610,2613,2616,2649,2654,2674,2693,2703,2707,2730,2738,2741,2749,2768,2784,2809,2821,2831,2835,2858,2866,2869,2877,2908,2911,2929,2947,2949,2958,2962,2969,2972,2974,2979,2984,2990,3024,3077,3086,3090,3114,3133,3160,3165,3168,3200,3205,3214,3218,3242,3253,3261,3293,3296,3313,3332,3342,3346,3389,3406,3412,3423,3450,3461,3482,3507,3517,3520,3585,3634,3648,3713,3716,3718,3724,3749,3751,3762,3773,3776,3782,3804,3840,3904,3913,3976,4096,4159,4176,4186,4193,4197,4206,4213,4238,4256,4295,4301,4304,4348,4682,4688,4696,4698,4704,4746,4752,4786,4792,4800,4802,4808,4824,4882,4888,4992,5024,5112,5121,5743,5761,5792,5873,5888,5919,5952,5984,5998,6016,6103,6108,6176,6272,6279,6314,6320,6400,6480,6512,6528,6576,6656,6688,6823,6917,6981,7043,7086,7098,7168,7245,7258,7296,7312,7357,7401,7406,7413,7418,7424,7680,7960,7968,8008,8016,8025,8027,8029,8031,8064,8118,8126,8130,8134,8144,8150,8160,8178,8182,8305,8319,8336,8450,8455,8458,8469,8473,8484,8486,8488,8490,8495,8508,8517,8526,8579,11264,11499,11506,11520,11559,11565,11568,11631,11648,11680,11688,11696,11704,11712,11720,11728,11736,11823,12293,12337,12347,12353,12445,12449,12540,12549,12593,12704,12784,13312,19968,42192,42240,42512,42538,42560,42623,42656,42775,42786,42891,42960,42963,42965,42994,43011,43015,43020,43072,43138,43250,43259,43261,43274,43312,43360,43396,43471,43488,43494,43514,43520,43584,43588,43616,43642,43646,43697,43701,43705,43712,43714,43739,43744,43762,43777,43785,43793,43808,43816,43824,43868,43888,44032,55216,55243,63744,64112,64256,64275,64285,64287,64298,64312,64318,64320,64323,64326,64467,64848,64914,65008,65136,65142,65313,65345,65382,65474,65482,65490,65498,65536,65549,65576,65596,65599,65616,65664,66176,66208,66304,66349,66370,66384,66432,66464,66504,66560,66736,66776,66816,66864,66928,66940,66956,66964,66967,66979,66995,67003,67008,67072,67392,67424,67456,67463,67506,67584,67592,67594,67639,67644,67647,67680,67712,67808,67828,67840,67872,67968,68030,68096,68112,68117,68121,68192,68224,68288,68297,68352,68416,68448,68480,68608,68736,68800,68864,68938,68975,69248,69296,69314,69376,69415,69424,69488,69552,69600,69635,69745,69749,69763,69840,69891,69956,69959,69968,70006,70019,70081,70106,70108,70144,70163,70207,70272,70280,70282,70287,70303,70320,70405,70415,70419,70442,70450,70453,70461,70480,70493,70528,70539,70542,70544,70583,70609,70611,70656,70727,70751,70784,70852,70855,71040,71128,71168,71236,71296,71352,71424,71488,71680,71840,71935,71945,71948,71957,71960,71999,72001,72096,72106,72161,72163,72192,72203,72250,72272,72284,72349,72368,72640,72704,72714,72768,72818,72960,72968,72971,73030,73056,73063,73066,73112,73440,73474,73476,73490,73648,73728,74880,77712,77824,78913,78944,82944,90368,92160,92736,92784,92880,92928,92992,93027,93053,93504,93760,93952,94032,94099,94176,94179,94208,100352,101631,110576,110581,110589,110592,110898,110928,110933,110948,110960,113664,113776,113792,113808,119808,119894,119966,119970,119973,119977,119982,119995,119997,120005,120071,120077,120086,120094,120123,120128,120134,120138,120146,120488,120514,120540,120572,120598,120630,120656,120688,120714,120746,120772,122624,122661,122928,123136,123191,123214,123536,123584,124112,124368,124400,124896,124904,124909,124912,124928,125184,125259,126464,126469,126497,126500,126503,126505,126516,126521,126523,126530,126535,126537,126539,126541,126545,126548,126551,126553,126555,126557,126559,126561,126564,126567,126572,126580,126585,126590,126592,126603,126625,126629,126635,131072,173824,177984,178208,183984,191472,194560,196608,201552"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("90,122,170,181,186,214,246,705,721,740,748,750,884,887,893,895,902,906,908,929,1013,1153,1327,1366,1369,1416,1514,1522,1610,1647,1747,1749,1766,1775,1788,1791,1808,1839,1957,1969,2026,2037,2042,2069,2074,2084,2088,2136,2154,2183,2190,2249,2361,2365,2384,2401,2432,2444,2448,2472,2480,2482,2489,2493,2510,2525,2529,2545,2556,2570,2576,2600,2608,2611,2614,2617,2652,2654,2676,2701,2705,2728,2736,2739,2745,2749,2768,2785,2809,2828,2832,2856,2864,2867,2873,2877,2909,2913,2929,2947,2954,2960,2965,2970,2972,2975,2980,2986,3001,3024,3084,3088,3112,3129,3133,3162,3165,3169,3200,3212,3216,3240,3251,3257,3261,3294,3297,3314,3340,3344,3386,3389,3406,3414,3425,3455,3478,3505,3515,3517,3526,3632,3635,3654,3714,3716,3722,3747,3749,3760,3763,3773,3780,3782,3807,3840,3911,3948,3980,4138,4159,4181,4189,4193,4198,4208,4225,4238,4293,4295,4301,4346,4680,4685,4694,4696,4701,4744,4749,4784,4789,4798,4800,4805,4822,4880,4885,4954,5007,5109,5117,5740,5759,5786,5866,5880,5905,5937,5969,5996,6000,6067,6103,6108,6264,6276,6312,6314,6389,6430,6509,6516,6571,6601,6678,6740,6823,6963,6988,7072,7087,7141,7203,7247,7293,7306,7354,7359,7404,7411,7414,7418,7615,7957,7965,8005,8013,8023,8025,8027,8029,8061,8116,8124,8126,8132,8140,8147,8155,8172,8180,8188,8305,8319,8348,8450,8455,8467,8469,8477,8484,8486,8488,8493,8505,8511,8521,8526,8580,11492,11502,11507,11557,11559,11565,11623,11631,11670,11686,11694,11702,11710,11718,11726,11734,11742,11823,12294,12341,12348,12438,12447,12538,12543,12591,12686,12735,12799,19903,42124,42237,42508,42527,42539,42606,42653,42725,42783,42888,42957,42961,42963,42972,43009,43013,43018,43042,43123,43187,43255,43259,43262,43301,43334,43388,43442,43471,43492,43503,43518,43560,43586,43595,43638,43642,43695,43697,43702,43709,43712,43714,43741,43754,43764,43782,43790,43798,43814,43822,43866,43881,44002,55203,55238,55291,64109,64217,64262,64279,64285,64296,64310,64316,64318,64321,64324,64433,64829,64911,64967,65019,65140,65276,65338,65370,65470,65479,65487,65495,65500,65547,65574,65594,65597,65613,65629,65786,66204,66256,66335,66368,66377,66421,66461,66499,66511,66717,66771,66811,66855,66915,66938,66954,66962,66965,66977,66993,67001,67004,67059,67382,67413,67431,67461,67504,67514,67589,67592,67637,67640,67644,67669,67702,67742,67826,67829,67861,67897,68023,68031,68096,68115,68119,68149,68220,68252,68295,68324,68405,68437,68466,68497,68680,68786,68850,68899,68965,68997,69289,69297,69316,69404,69415,69445,69505,69572,69622,69687,69746,69749,69807,69864,69926,69956,69959,70002,70006,70066,70084,70106,70108,70161,70187,70208,70278,70280,70285,70301,70312,70366,70412,70416,70440,70448,70451,70457,70461,70480,70497,70537,70539,70542,70581,70583,70609,70611,70708,70730,70753,70831,70853,70855,71086,71131,71215,71236,71338,71352,71450,71494,71723,71903,71942,71945,71955,71958,71983,71999,72001,72103,72144,72161,72163,72192,72242,72250,72272,72329,72349,72440,72672,72712,72750,72768,72847,72966,72969,73008,73030,73061,73064,73097,73112,73458,73474,73488,73523,73648,74649,75075,77808,78895,78918,82938,83526,90397,92728,92766,92862,92909,92975,92995,93047,93071,93548,93823,94026,94032,94111,94177,94179,100343,101589,101640,110579,110587,110590,110882,110898,110930,110933,110951,111355,113770,113788,113800,113817,119892,119964,119967,119970,119974,119980,119993,119995,120003,120069,120074,120084,120092,120121,120126,120132,120134,120144,120485,120512,120538,120570,120596,120628,120654,120686,120712,120744,120770,120779,122654,122666,122989,123180,123197,123214,123565,123627,124139,124397,124400,124902,124907,124910,124926,125124,125251,125259,126467,126495,126498,126500,126503,126514,126519,126521,126523,126530,126535,126537,126539,126543,126546,126548,126551,126553,126555,126557,126559,126562,126564,126570,126578,126583,126588,126590,126601,126619,126627,126633,126651,173791,177977,178205,183969,191456,192093,195101,201546,205743"); } LIT.with(|lit| lit.clone()) };
        US21::US21_Some(v7, v8)
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lu"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("uppercaseletter"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v12));
        let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v13));
        let v15: bool = named_170(v0.clone(), v14);
        if v15 {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("65,192,216,256,258,260,262,264,266,268,270,272,274,276,278,280,282,284,286,288,290,292,294,296,298,300,302,304,306,308,310,313,315,317,319,321,323,325,327,330,332,334,336,338,340,342,344,346,348,350,352,354,356,358,360,362,364,366,368,370,372,374,376,379,381,385,388,390,393,398,403,406,412,415,418,420,422,425,428,430,433,437,439,444,452,455,458,461,463,465,467,469,471,473,475,478,480,482,484,486,488,490,492,494,497,500,502,506,508,510,512,514,516,518,520,522,524,526,528,530,532,534,536,538,540,542,544,546,548,550,552,554,556,558,560,562,570,573,577,579,584,586,588,590,880,882,886,895,902,904,908,910,913,931,975,978,984,986,988,990,992,994,996,998,1000,1002,1004,1006,1012,1015,1017,1021,1120,1122,1124,1126,1128,1130,1132,1134,1136,1138,1140,1142,1144,1146,1148,1150,1152,1162,1164,1166,1168,1170,1172,1174,1176,1178,1180,1182,1184,1186,1188,1190,1192,1194,1196,1198,1200,1202,1204,1206,1208,1210,1212,1214,1216,1219,1221,1223,1225,1227,1229,1232,1234,1236,1238,1240,1242,1244,1246,1248,1250,1252,1254,1256,1258,1260,1262,1264,1266,1268,1270,1272,1274,1276,1278,1280,1282,1284,1286,1288,1290,1292,1294,1296,1298,1300,1302,1304,1306,1308,1310,1312,1314,1316,1318,1320,1322,1324,1326,1329,4256,4295,4301,5024,7305,7312,7357,7680,7682,7684,7686,7688,7690,7692,7694,7696,7698,7700,7702,7704,7706,7708,7710,7712,7714,7716,7718,7720,7722,7724,7726,7728,7730,7732,7734,7736,7738,7740,7742,7744,7746,7748,7750,7752,7754,7756,7758,7760,7762,7764,7766,7768,7770,7772,7774,7776,7778,7780,7782,7784,7786,7788,7790,7792,7794,7796,7798,7800,7802,7804,7806,7808,7810,7812,7814,7816,7818,7820,7822,7824,7826,7828,7838,7840,7842,7844,7846,7848,7850,7852,7854,7856,7858,7860,7862,7864,7866,7868,7870,7872,7874,7876,7878,7880,7882,7884,7886,7888,7890,7892,7894,7896,7898,7900,7902,7904,7906,7908,7910,7912,7914,7916,7918,7920,7922,7924,7926,7928,7930,7932,7934,7944,7960,7976,7992,8008,8025,8027,8029,8031,8040,8120,8136,8152,8168,8184,8450,8455,8459,8464,8469,8473,8484,8486,8488,8490,8496,8510,8517,8579,11264,11360,11362,11367,11369,11371,11373,11378,11381,11390,11394,11396,11398,11400,11402,11404,11406,11408,11410,11412,11414,11416,11418,11420,11422,11424,11426,11428,11430,11432,11434,11436,11438,11440,11442,11444,11446,11448,11450,11452,11454,11456,11458,11460,11462,11464,11466,11468,11470,11472,11474,11476,11478,11480,11482,11484,11486,11488,11490,11499,11501,11506,42560,42562,42564,42566,42568,42570,42572,42574,42576,42578,42580,42582,42584,42586,42588,42590,42592,42594,42596,42598,42600,42602,42604,42624,42626,42628,42630,42632,42634,42636,42638,42640,42642,42644,42646,42648,42650,42786,42788,42790,42792,42794,42796,42798,42802,42804,42806,42808,42810,42812,42814,42816,42818,42820,42822,42824,42826,42828,42830,42832,42834,42836,42838,42840,42842,42844,42846,42848,42850,42852,42854,42856,42858,42860,42862,42873,42875,42877,42880,42882,42884,42886,42891,42893,42896,42898,42902,42904,42906,42908,42910,42912,42914,42916,42918,42920,42922,42928,42934,42936,42938,42940,42942,42944,42946,42948,42953,42955,42960,42966,42968,42970,42972,42997,65313,66560,66736,66928,66940,66956,66964,68736,68944,71840,93760,119808,119860,119912,119964,119966,119970,119973,119977,119982,120016,120068,120071,120077,120086,120120,120123,120128,120134,120138,120172,120224,120276,120328,120380,120432,120488,120546,120604,120662,120720,120778,125184"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("90,214,222,256,258,260,262,264,266,268,270,272,274,276,278,280,282,284,286,288,290,292,294,296,298,300,302,304,306,308,310,313,315,317,319,321,323,325,327,330,332,334,336,338,340,342,344,346,348,350,352,354,356,358,360,362,364,366,368,370,372,374,377,379,381,386,388,391,395,401,404,408,413,416,418,420,423,425,428,431,435,437,440,444,452,455,458,461,463,465,467,469,471,473,475,478,480,482,484,486,488,490,492,494,497,500,504,506,508,510,512,514,516,518,520,522,524,526,528,530,532,534,536,538,540,542,544,546,548,550,552,554,556,558,560,562,571,574,577,582,584,586,588,590,880,882,886,895,902,906,908,911,929,939,975,980,984,986,988,990,992,994,996,998,1000,1002,1004,1006,1012,1015,1018,1071,1120,1122,1124,1126,1128,1130,1132,1134,1136,1138,1140,1142,1144,1146,1148,1150,1152,1162,1164,1166,1168,1170,1172,1174,1176,1178,1180,1182,1184,1186,1188,1190,1192,1194,1196,1198,1200,1202,1204,1206,1208,1210,1212,1214,1217,1219,1221,1223,1225,1227,1229,1232,1234,1236,1238,1240,1242,1244,1246,1248,1250,1252,1254,1256,1258,1260,1262,1264,1266,1268,1270,1272,1274,1276,1278,1280,1282,1284,1286,1288,1290,1292,1294,1296,1298,1300,1302,1304,1306,1308,1310,1312,1314,1316,1318,1320,1322,1324,1326,1366,4293,4295,4301,5109,7305,7354,7359,7680,7682,7684,7686,7688,7690,7692,7694,7696,7698,7700,7702,7704,7706,7708,7710,7712,7714,7716,7718,7720,7722,7724,7726,7728,7730,7732,7734,7736,7738,7740,7742,7744,7746,7748,7750,7752,7754,7756,7758,7760,7762,7764,7766,7768,7770,7772,7774,7776,7778,7780,7782,7784,7786,7788,7790,7792,7794,7796,7798,7800,7802,7804,7806,7808,7810,7812,7814,7816,7818,7820,7822,7824,7826,7828,7838,7840,7842,7844,7846,7848,7850,7852,7854,7856,7858,7860,7862,7864,7866,7868,7870,7872,7874,7876,7878,7880,7882,7884,7886,7888,7890,7892,7894,7896,7898,7900,7902,7904,7906,7908,7910,7912,7914,7916,7918,7920,7922,7924,7926,7928,7930,7932,7934,7951,7965,7983,7999,8013,8025,8027,8029,8031,8047,8123,8139,8155,8172,8187,8450,8455,8461,8466,8469,8477,8484,8486,8488,8493,8499,8511,8517,8579,11311,11360,11364,11367,11369,11371,11376,11378,11381,11392,11394,11396,11398,11400,11402,11404,11406,11408,11410,11412,11414,11416,11418,11420,11422,11424,11426,11428,11430,11432,11434,11436,11438,11440,11442,11444,11446,11448,11450,11452,11454,11456,11458,11460,11462,11464,11466,11468,11470,11472,11474,11476,11478,11480,11482,11484,11486,11488,11490,11499,11501,11506,42560,42562,42564,42566,42568,42570,42572,42574,42576,42578,42580,42582,42584,42586,42588,42590,42592,42594,42596,42598,42600,42602,42604,42624,42626,42628,42630,42632,42634,42636,42638,42640,42642,42644,42646,42648,42650,42786,42788,42790,42792,42794,42796,42798,42802,42804,42806,42808,42810,42812,42814,42816,42818,42820,42822,42824,42826,42828,42830,42832,42834,42836,42838,42840,42842,42844,42846,42848,42850,42852,42854,42856,42858,42860,42862,42873,42875,42878,42880,42882,42884,42886,42891,42893,42896,42898,42902,42904,42906,42908,42910,42912,42914,42916,42918,42920,42926,42932,42934,42936,42938,42940,42942,42944,42946,42951,42953,42956,42960,42966,42968,42970,42972,42997,65338,66599,66771,66938,66954,66962,66965,68786,68965,71871,93791,119833,119885,119937,119964,119967,119970,119974,119980,119989,120041,120069,120074,120084,120092,120121,120126,120132,120134,120144,120197,120249,120301,120353,120405,120457,120512,120570,120628,120686,120744,120778,125217"); } LIT.with(|lit| lit.clone()) };
            US21::US21_Some(v16, v17)
        } else {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ll"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lowercaseletter"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v21));
            let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v22));
            let v24: bool = named_170(v0.clone(), v23);
            if v24 {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("97,181,223,248,257,259,261,263,265,267,269,271,273,275,277,279,281,283,285,287,289,291,293,295,297,299,301,303,305,307,309,311,314,316,318,320,322,324,326,328,331,333,335,337,339,341,343,345,347,349,351,353,355,357,359,361,363,365,367,369,371,373,375,378,380,382,387,389,392,396,402,405,409,414,417,419,421,424,426,429,432,436,438,441,445,454,457,460,462,464,466,468,470,472,474,476,479,481,483,485,487,489,491,493,495,499,501,505,507,509,511,513,515,517,519,521,523,525,527,529,531,533,535,537,539,541,543,545,547,549,551,553,555,557,559,561,563,572,575,578,583,585,587,589,591,661,881,883,887,891,912,940,976,981,985,987,989,991,993,995,997,999,1001,1003,1005,1007,1013,1016,1019,1072,1121,1123,1125,1127,1129,1131,1133,1135,1137,1139,1141,1143,1145,1147,1149,1151,1153,1163,1165,1167,1169,1171,1173,1175,1177,1179,1181,1183,1185,1187,1189,1191,1193,1195,1197,1199,1201,1203,1205,1207,1209,1211,1213,1215,1218,1220,1222,1224,1226,1228,1230,1233,1235,1237,1239,1241,1243,1245,1247,1249,1251,1253,1255,1257,1259,1261,1263,1265,1267,1269,1271,1273,1275,1277,1279,1281,1283,1285,1287,1289,1291,1293,1295,1297,1299,1301,1303,1305,1307,1309,1311,1313,1315,1317,1319,1321,1323,1325,1327,1376,4304,4349,5112,7296,7306,7424,7531,7545,7681,7683,7685,7687,7689,7691,7693,7695,7697,7699,7701,7703,7705,7707,7709,7711,7713,7715,7717,7719,7721,7723,7725,7727,7729,7731,7733,7735,7737,7739,7741,7743,7745,7747,7749,7751,7753,7755,7757,7759,7761,7763,7765,7767,7769,7771,7773,7775,7777,7779,7781,7783,7785,7787,7789,7791,7793,7795,7797,7799,7801,7803,7805,7807,7809,7811,7813,7815,7817,7819,7821,7823,7825,7827,7829,7839,7841,7843,7845,7847,7849,7851,7853,7855,7857,7859,7861,7863,7865,7867,7869,7871,7873,7875,7877,7879,7881,7883,7885,7887,7889,7891,7893,7895,7897,7899,7901,7903,7905,7907,7909,7911,7913,7915,7917,7919,7921,7923,7925,7927,7929,7931,7933,7935,7952,7968,7984,8000,8016,8032,8048,8064,8080,8096,8112,8118,8126,8130,8134,8144,8150,8160,8178,8182,8458,8462,8467,8495,8500,8505,8508,8518,8526,8580,11312,11361,11365,11368,11370,11372,11377,11379,11382,11393,11395,11397,11399,11401,11403,11405,11407,11409,11411,11413,11415,11417,11419,11421,11423,11425,11427,11429,11431,11433,11435,11437,11439,11441,11443,11445,11447,11449,11451,11453,11455,11457,11459,11461,11463,11465,11467,11469,11471,11473,11475,11477,11479,11481,11483,11485,11487,11489,11491,11500,11502,11507,11520,11559,11565,42561,42563,42565,42567,42569,42571,42573,42575,42577,42579,42581,42583,42585,42587,42589,42591,42593,42595,42597,42599,42601,42603,42605,42625,42627,42629,42631,42633,42635,42637,42639,42641,42643,42645,42647,42649,42651,42787,42789,42791,42793,42795,42797,42799,42803,42805,42807,42809,42811,42813,42815,42817,42819,42821,42823,42825,42827,42829,42831,42833,42835,42837,42839,42841,42843,42845,42847,42849,42851,42853,42855,42857,42859,42861,42863,42865,42874,42876,42879,42881,42883,42885,42887,42892,42894,42897,42899,42903,42905,42907,42909,42911,42913,42915,42917,42919,42921,42927,42933,42935,42937,42939,42941,42943,42945,42947,42952,42954,42957,42961,42963,42965,42967,42969,42971,42998,43002,43824,43872,43888,64256,64275,65345,66600,66776,66967,66979,66995,67003,68800,68976,71872,93792,119834,119886,119894,119938,119990,119995,119997,120005,120042,120094,120146,120198,120250,120302,120354,120406,120458,120514,120540,120572,120598,120630,120656,120688,120714,120746,120772,120779,122624,122635,122661,125218"); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("122,181,246,255,257,259,261,263,265,267,269,271,273,275,277,279,281,283,285,287,289,291,293,295,297,299,301,303,305,307,309,312,314,316,318,320,322,324,326,329,331,333,335,337,339,341,343,345,347,349,351,353,355,357,359,361,363,365,367,369,371,373,375,378,380,384,387,389,392,397,402,405,411,414,417,419,421,424,427,429,432,436,438,442,447,454,457,460,462,464,466,468,470,472,474,477,479,481,483,485,487,489,491,493,496,499,501,505,507,509,511,513,515,517,519,521,523,525,527,529,531,533,535,537,539,541,543,545,547,549,551,553,555,557,559,561,569,572,576,578,583,585,587,589,659,687,881,883,887,893,912,974,977,983,985,987,989,991,993,995,997,999,1001,1003,1005,1011,1013,1016,1020,1119,1121,1123,1125,1127,1129,1131,1133,1135,1137,1139,1141,1143,1145,1147,1149,1151,1153,1163,1165,1167,1169,1171,1173,1175,1177,1179,1181,1183,1185,1187,1189,1191,1193,1195,1197,1199,1201,1203,1205,1207,1209,1211,1213,1215,1218,1220,1222,1224,1226,1228,1231,1233,1235,1237,1239,1241,1243,1245,1247,1249,1251,1253,1255,1257,1259,1261,1263,1265,1267,1269,1271,1273,1275,1277,1279,1281,1283,1285,1287,1289,1291,1293,1295,1297,1299,1301,1303,1305,1307,1309,1311,1313,1315,1317,1319,1321,1323,1325,1327,1416,4346,4351,5117,7304,7306,7467,7543,7578,7681,7683,7685,7687,7689,7691,7693,7695,7697,7699,7701,7703,7705,7707,7709,7711,7713,7715,7717,7719,7721,7723,7725,7727,7729,7731,7733,7735,7737,7739,7741,7743,7745,7747,7749,7751,7753,7755,7757,7759,7761,7763,7765,7767,7769,7771,7773,7775,7777,7779,7781,7783,7785,7787,7789,7791,7793,7795,7797,7799,7801,7803,7805,7807,7809,7811,7813,7815,7817,7819,7821,7823,7825,7827,7837,7839,7841,7843,7845,7847,7849,7851,7853,7855,7857,7859,7861,7863,7865,7867,7869,7871,7873,7875,7877,7879,7881,7883,7885,7887,7889,7891,7893,7895,7897,7899,7901,7903,7905,7907,7909,7911,7913,7915,7917,7919,7921,7923,7925,7927,7929,7931,7933,7943,7957,7975,7991,8005,8023,8039,8061,8071,8087,8103,8116,8119,8126,8132,8135,8147,8151,8167,8180,8183,8458,8463,8467,8495,8500,8505,8509,8521,8526,8580,11359,11361,11366,11368,11370,11372,11377,11380,11387,11393,11395,11397,11399,11401,11403,11405,11407,11409,11411,11413,11415,11417,11419,11421,11423,11425,11427,11429,11431,11433,11435,11437,11439,11441,11443,11445,11447,11449,11451,11453,11455,11457,11459,11461,11463,11465,11467,11469,11471,11473,11475,11477,11479,11481,11483,11485,11487,11489,11492,11500,11502,11507,11557,11559,11565,42561,42563,42565,42567,42569,42571,42573,42575,42577,42579,42581,42583,42585,42587,42589,42591,42593,42595,42597,42599,42601,42603,42605,42625,42627,42629,42631,42633,42635,42637,42639,42641,42643,42645,42647,42649,42651,42787,42789,42791,42793,42795,42797,42801,42803,42805,42807,42809,42811,42813,42815,42817,42819,42821,42823,42825,42827,42829,42831,42833,42835,42837,42839,42841,42843,42845,42847,42849,42851,42853,42855,42857,42859,42861,42863,42872,42874,42876,42879,42881,42883,42885,42887,42892,42894,42897,42901,42903,42905,42907,42909,42911,42913,42915,42917,42919,42921,42927,42933,42935,42937,42939,42941,42943,42945,42947,42952,42954,42957,42961,42963,42965,42967,42969,42971,42998,43002,43866,43880,43967,64262,64279,65370,66639,66811,66977,66993,67001,67004,68850,68997,71903,93823,119859,119892,119911,119963,119993,119995,120003,120015,120067,120119,120171,120223,120275,120327,120379,120431,120485,120538,120545,120596,120603,120654,120661,120712,120719,120770,120777,120779,122633,122654,122666,125251"); } LIT.with(|lit| lit.clone()) };
                US21::US21_Some(v25, v26)
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lt"); } LIT.with(|lit| lit.clone()) };
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("titlecaseletter"); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v30));
                let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28, v31));
                let v33: bool = named_170(v0.clone(), v32);
                if v33 {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("453,456,459,498,8072,8088,8104,8124,8140,8188"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("453,456,459,498,8079,8095,8111,8124,8140,8188"); } LIT.with(|lit| lit.clone()) };
                    US21::US21_Some(v34, v35)
                } else {
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lm"); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modifierletter"); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v40: Rc<UH0> = Rc::new(UH0::UH0_Cons(v38, v39));
                    let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v40));
                    let v42: bool = named_170(v0.clone(), v41);
                    if v42 {
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("688,710,736,748,750,884,890,1369,1600,1765,2036,2042,2074,2084,2088,2249,2417,3654,3782,4348,6103,6211,6823,7288,7468,7544,7579,8305,8319,8336,11388,11631,11823,12293,12337,12347,12445,12540,40981,42232,42508,42623,42652,42775,42864,42888,42994,43000,43471,43494,43632,43741,43763,43868,43881,65392,65438,67456,67463,67506,68942,68975,92992,93504,93547,94099,94176,94179,110576,110581,110589,122928,123191,124139,125259"); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("705,721,740,748,750,884,890,1369,1600,1766,2037,2042,2074,2084,2088,2249,2417,3654,3782,4348,6103,6211,6823,7293,7530,7544,7615,8305,8319,8348,11389,11631,11823,12293,12341,12347,12446,12542,40981,42237,42508,42623,42653,42783,42864,42888,42996,43001,43471,43494,43632,43741,43764,43871,43881,65392,65439,67461,67504,67514,68942,68975,92995,93506,93548,94111,94177,94179,110579,110587,110590,122989,123197,124139,125259"); } LIT.with(|lit| lit.clone()) };
                        US21::US21_Some(v43, v44)
                    } else {
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lo"); } LIT.with(|lit| lit.clone()) };
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("otherletter"); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v47, v48));
                        let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v46, v49));
                        let v51: bool = named_170(v0.clone(), v50);
                        if v51 {
                            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("170,186,443,448,660,1488,1519,1568,1601,1646,1649,1749,1774,1786,1791,1808,1810,1869,1969,1994,2048,2112,2144,2160,2185,2208,2308,2365,2384,2392,2418,2437,2447,2451,2474,2482,2486,2493,2510,2524,2527,2544,2556,2565,2575,2579,2602,2610,2613,2616,2649,2654,2674,2693,2703,2707,2730,2738,2741,2749,2768,2784,2809,2821,2831,2835,2858,2866,2869,2877,2908,2911,2929,2947,2949,2958,2962,2969,2972,2974,2979,2984,2990,3024,3077,3086,3090,3114,3133,3160,3165,3168,3200,3205,3214,3218,3242,3253,3261,3293,3296,3313,3332,3342,3346,3389,3406,3412,3423,3450,3461,3482,3507,3517,3520,3585,3634,3648,3713,3716,3718,3724,3749,3751,3762,3773,3776,3804,3840,3904,3913,3976,4096,4159,4176,4186,4193,4197,4206,4213,4238,4352,4682,4688,4696,4698,4704,4746,4752,4786,4792,4800,4802,4808,4824,4882,4888,4992,5121,5743,5761,5792,5873,5888,5919,5952,5984,5998,6016,6108,6176,6212,6272,6279,6314,6320,6400,6480,6512,6528,6576,6656,6688,6917,6981,7043,7086,7098,7168,7245,7258,7401,7406,7413,7418,8501,11568,11648,11680,11688,11696,11704,11712,11720,11728,11736,12294,12348,12353,12447,12449,12543,12549,12593,12704,12784,13312,19968,40982,42192,42240,42512,42538,42606,42656,42895,42999,43003,43011,43015,43020,43072,43138,43250,43259,43261,43274,43312,43360,43396,43488,43495,43514,43520,43584,43588,43616,43633,43642,43646,43697,43701,43705,43712,43714,43739,43744,43762,43777,43785,43793,43808,43816,43968,44032,55216,55243,63744,64112,64285,64287,64298,64312,64318,64320,64323,64326,64467,64848,64914,65008,65136,65142,65382,65393,65440,65474,65482,65490,65498,65536,65549,65576,65596,65599,65616,65664,66176,66208,66304,66349,66370,66384,66432,66464,66504,66640,66816,66864,67008,67072,67392,67424,67584,67592,67594,67639,67644,67647,67680,67712,67808,67828,67840,67872,67968,68030,68096,68112,68117,68121,68192,68224,68288,68297,68352,68416,68448,68480,68608,68864,68938,68943,69248,69296,69314,69376,69415,69424,69488,69552,69600,69635,69745,69749,69763,69840,69891,69956,69959,69968,70006,70019,70081,70106,70108,70144,70163,70207,70272,70280,70282,70287,70303,70320,70405,70415,70419,70442,70450,70453,70461,70480,70493,70528,70539,70542,70544,70583,70609,70611,70656,70727,70751,70784,70852,70855,71040,71128,71168,71236,71296,71352,71424,71488,71680,71935,71945,71948,71957,71960,71999,72001,72096,72106,72161,72163,72192,72203,72250,72272,72284,72349,72368,72640,72704,72714,72768,72818,72960,72968,72971,73030,73056,73063,73066,73112,73440,73474,73476,73490,73648,73728,74880,77712,77824,78913,78944,82944,90368,92160,92736,92784,92880,92928,93027,93053,93507,93952,94032,94208,100352,101631,110592,110898,110928,110933,110948,110960,113664,113776,113792,113808,122634,123136,123214,123536,123584,124112,124368,124400,124896,124904,124909,124912,124928,126464,126469,126497,126500,126503,126505,126516,126521,126523,126530,126535,126537,126539,126541,126545,126548,126551,126553,126555,126557,126559,126561,126564,126567,126572,126580,126585,126590,126592,126603,126625,126629,126635,131072,173824,177984,178208,183984,191472,194560,196608,201552"); } LIT.with(|lit| lit.clone()) };
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("170,186,443,451,660,1514,1522,1599,1610,1647,1747,1749,1775,1788,1791,1808,1839,1957,1969,2026,2069,2136,2154,2183,2190,2248,2361,2365,2384,2401,2432,2444,2448,2472,2480,2482,2489,2493,2510,2525,2529,2545,2556,2570,2576,2600,2608,2611,2614,2617,2652,2654,2676,2701,2705,2728,2736,2739,2745,2749,2768,2785,2809,2828,2832,2856,2864,2867,2873,2877,2909,2913,2929,2947,2954,2960,2965,2970,2972,2975,2980,2986,3001,3024,3084,3088,3112,3129,3133,3162,3165,3169,3200,3212,3216,3240,3251,3257,3261,3294,3297,3314,3340,3344,3386,3389,3406,3414,3425,3455,3478,3505,3515,3517,3526,3632,3635,3653,3714,3716,3722,3747,3749,3760,3763,3773,3780,3807,3840,3911,3948,3980,4138,4159,4181,4189,4193,4198,4208,4225,4238,4680,4685,4694,4696,4701,4744,4749,4784,4789,4798,4800,4805,4822,4880,4885,4954,5007,5740,5759,5786,5866,5880,5905,5937,5969,5996,6000,6067,6108,6210,6264,6276,6312,6314,6389,6430,6509,6516,6571,6601,6678,6740,6963,6988,7072,7087,7141,7203,7247,7287,7404,7411,7414,7418,8504,11623,11670,11686,11694,11702,11710,11718,11726,11734,11742,12294,12348,12438,12447,12538,12543,12591,12686,12735,12799,19903,40980,42124,42231,42507,42527,42539,42606,42725,42895,42999,43009,43013,43018,43042,43123,43187,43255,43259,43262,43301,43334,43388,43442,43492,43503,43518,43560,43586,43595,43631,43638,43642,43695,43697,43702,43709,43712,43714,43740,43754,43762,43782,43790,43798,43814,43822,44002,55203,55238,55291,64109,64217,64285,64296,64310,64316,64318,64321,64324,64433,64829,64911,64967,65019,65140,65276,65391,65437,65470,65479,65487,65495,65500,65547,65574,65594,65597,65613,65629,65786,66204,66256,66335,66368,66377,66421,66461,66499,66511,66717,66855,66915,67059,67382,67413,67431,67589,67592,67637,67640,67644,67669,67702,67742,67826,67829,67861,67897,68023,68031,68096,68115,68119,68149,68220,68252,68295,68324,68405,68437,68466,68497,68680,68899,68941,68943,69289,69297,69316,69404,69415,69445,69505,69572,69622,69687,69746,69749,69807,69864,69926,69956,69959,70002,70006,70066,70084,70106,70108,70161,70187,70208,70278,70280,70285,70301,70312,70366,70412,70416,70440,70448,70451,70457,70461,70480,70497,70537,70539,70542,70581,70583,70609,70611,70708,70730,70753,70831,70853,70855,71086,71131,71215,71236,71338,71352,71450,71494,71723,71942,71945,71955,71958,71983,71999,72001,72103,72144,72161,72163,72192,72242,72250,72272,72329,72349,72440,72672,72712,72750,72768,72847,72966,72969,73008,73030,73061,73064,73097,73112,73458,73474,73488,73523,73648,74649,75075,77808,78895,78918,82938,83526,90397,92728,92766,92862,92909,92975,93047,93071,93546,94026,94032,100343,101589,101640,110882,110898,110930,110933,110951,111355,113770,113788,113800,113817,122634,123180,123214,123565,123627,124138,124397,124400,124902,124907,124910,124926,125124,126467,126495,126498,126500,126503,126514,126519,126521,126523,126530,126535,126537,126539,126543,126546,126548,126551,126553,126555,126557,126559,126562,126564,126570,126578,126583,126588,126590,126601,126619,126627,126633,126651,173791,177977,178205,183969,191456,192093,195101,201546,205743"); } LIT.with(|lit| lit.clone()) };
                            US21::US21_Some(v52, v53)
                        } else {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mark"); } LIT.with(|lit| lit.clone()) };
                            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("combiningmark"); } LIT.with(|lit| lit.clone()) };
                            let v58: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v59: Rc<UH0> = Rc::new(UH0::UH0_Cons(v57, v58));
                            let v60: Rc<UH0> = Rc::new(UH0::UH0_Cons(v56, v59));
                            let v61: Rc<UH0> = Rc::new(UH0::UH0_Cons(v55, v60));
                            let v62: bool = named_170(v0.clone(), v61);
                            if v62 {
                                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("768,1155,1425,1471,1473,1476,1479,1552,1611,1648,1750,1759,1767,1770,1809,1840,1958,2027,2045,2070,2075,2085,2089,2137,2199,2250,2275,2362,2366,2385,2402,2433,2492,2494,2503,2507,2519,2530,2558,2561,2620,2622,2631,2635,2641,2672,2677,2689,2748,2750,2759,2763,2786,2810,2817,2876,2878,2887,2891,2901,2914,2946,3006,3014,3018,3031,3072,3132,3134,3142,3146,3157,3170,3201,3260,3262,3270,3274,3285,3298,3315,3328,3387,3390,3398,3402,3415,3426,3457,3530,3535,3542,3544,3570,3633,3636,3655,3761,3764,3784,3864,3893,3895,3897,3902,3953,3974,3981,3993,4038,4139,4182,4190,4194,4199,4209,4226,4239,4250,4957,5906,5938,5970,6002,6068,6109,6155,6159,6277,6313,6432,6448,6679,6741,6752,6783,6832,6912,6964,7019,7040,7073,7142,7204,7376,7380,7405,7412,7415,7616,8400,11503,11647,11744,12330,12441,42607,42612,42654,42736,43010,43014,43019,43043,43052,43136,43188,43232,43263,43302,43335,43392,43443,43493,43561,43587,43596,43643,43696,43698,43703,43710,43713,43755,43765,44003,44012,64286,65024,65056,66045,66272,66422,68097,68101,68108,68152,68159,68325,68900,68969,69291,69372,69446,69506,69632,69688,69744,69747,69759,69808,69826,69888,69927,69957,70003,70016,70067,70089,70094,70188,70206,70209,70367,70400,70459,70462,70471,70475,70487,70498,70502,70512,70584,70594,70597,70599,70604,70610,70625,70709,70750,70832,71087,71096,71132,71216,71339,71453,71724,71984,71991,71995,72000,72002,72145,72154,72164,72193,72243,72251,72263,72273,72330,72751,72760,72850,72873,73009,73018,73020,73023,73031,73098,73104,73107,73459,73472,73475,73524,73534,73562,78912,78919,90398,92912,92976,94031,94033,94095,94180,94192,113821,118528,118576,119141,119149,119163,119173,119210,119362,121344,121403,121461,121476,121499,121505,122880,122888,122907,122915,122918,123023,123184,123566,123628,124140,124398,125136,125252,917760"); } LIT.with(|lit| lit.clone()) };
                                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("879,1161,1469,1471,1474,1477,1479,1562,1631,1648,1756,1764,1768,1773,1809,1866,1968,2035,2045,2073,2083,2087,2093,2139,2207,2273,2307,2364,2383,2391,2403,2435,2492,2500,2504,2509,2519,2531,2558,2563,2620,2626,2632,2637,2641,2673,2677,2691,2748,2757,2761,2765,2787,2815,2819,2876,2884,2888,2893,2903,2915,2946,3010,3016,3021,3031,3076,3132,3140,3144,3149,3158,3171,3203,3260,3268,3272,3277,3286,3299,3315,3331,3388,3396,3400,3405,3415,3427,3459,3530,3540,3542,3551,3571,3633,3642,3662,3761,3772,3790,3865,3893,3895,3897,3903,3972,3975,3991,4028,4038,4158,4185,4192,4196,4205,4212,4237,4239,4253,4959,5909,5940,5971,6003,6099,6109,6157,6159,6278,6313,6443,6459,6683,6750,6780,6783,6862,6916,6980,7027,7042,7085,7155,7223,7378,7400,7405,7412,7417,7679,8432,11505,11647,11775,12335,12442,42610,42621,42655,42737,43010,43014,43019,43047,43052,43137,43205,43249,43263,43309,43347,43395,43456,43493,43574,43587,43597,43645,43696,43700,43704,43711,43713,43759,43766,44010,44013,64286,65039,65071,66045,66272,66426,68099,68102,68111,68154,68159,68326,68903,68973,69292,69375,69456,69509,69634,69702,69744,69748,69762,69818,69826,69890,69940,69958,70003,70018,70080,70092,70095,70199,70206,70209,70378,70403,70460,70468,70472,70477,70487,70499,70508,70516,70592,70594,70597,70602,70608,70610,70626,70726,70750,70851,71093,71104,71133,71232,71351,71467,71738,71989,71992,71998,72000,72003,72151,72160,72164,72202,72249,72254,72263,72283,72345,72758,72767,72871,72886,73014,73018,73021,73029,73031,73102,73105,73111,73462,73473,73475,73530,73538,73562,78912,78933,90415,92916,92982,94031,94087,94098,94180,94193,113822,118573,118598,119145,119154,119170,119179,119213,119364,121398,121452,121461,121476,121503,121519,122886,122904,122913,122916,122922,123023,123190,123566,123631,124143,124399,125142,125258,917999"); } LIT.with(|lit| lit.clone()) };
                                US21::US21_Some(v63, v64)
                            } else {
                                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("n"); } LIT.with(|lit| lit.clone()) };
                                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("number"); } LIT.with(|lit| lit.clone()) };
                                let v68: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let v69: Rc<UH0> = Rc::new(UH0::UH0_Cons(v67, v68));
                                let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v66, v69));
                                let v71: bool = named_170(v0.clone(), v70);
                                if v71 {
                                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("48,178,185,188,1632,1776,1984,2406,2534,2548,2662,2790,2918,2930,3046,3174,3192,3302,3416,3430,3558,3664,3792,3872,4160,4240,4969,5870,6112,6128,6160,6470,6608,6784,6800,6992,7088,7232,7248,8304,8308,8320,8528,8581,9312,9450,10102,11517,12295,12321,12344,12690,12832,12872,12881,12928,12977,42528,42726,43056,43216,43264,43472,43504,43600,44016,65296,65799,65856,65930,66273,66336,66369,66378,66513,66720,67672,67705,67751,67835,67862,68028,68032,68050,68160,68221,68253,68331,68440,68472,68521,68858,68912,68928,69216,69405,69457,69573,69714,69872,69942,70096,70113,70384,70736,70864,71248,71360,71376,71472,71904,72016,72688,72784,73040,73120,73552,73664,74752,90416,92768,92864,93008,93019,93552,93824,118000,119488,119520,119648,120782,123200,123632,124144,124401,125127,125264,126065,126125,126129,126209,126255,127232,130032"); } LIT.with(|lit| lit.clone()) };
                                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("57,179,185,190,1641,1785,1993,2415,2543,2553,2671,2799,2927,2935,3058,3183,3198,3311,3422,3448,3567,3673,3801,3891,4169,4249,4988,5872,6121,6137,6169,6479,6618,6793,6809,7001,7097,7241,7257,8304,8313,8329,8578,8585,9371,9471,10131,11517,12295,12329,12346,12693,12841,12879,12895,12937,12991,42537,42735,43061,43225,43273,43481,43513,43609,44025,65305,65843,65912,65931,66299,66339,66369,66378,66517,66729,67679,67711,67759,67839,67867,68029,68047,68095,68168,68222,68255,68335,68447,68479,68527,68863,68921,68937,69246,69414,69460,69579,69743,69881,69951,70105,70132,70393,70745,70873,71257,71369,71395,71483,71922,72025,72697,72812,73049,73129,73561,73684,74862,90425,92777,92873,93017,93025,93561,93846,118009,119507,119539,119672,120831,123209,123641,124153,124410,125135,125273,126123,126127,126132,126253,126269,127244,130041"); } LIT.with(|lit| lit.clone()) };
                                    US21::US21_Some(v72, v73)
                                } else {
                                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nd"); } LIT.with(|lit| lit.clone()) };
                                    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decimalnumber"); } LIT.with(|lit| lit.clone()) };
                                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("digit"); } LIT.with(|lit| lit.clone()) };
                                    let v78: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v79: Rc<UH0> = Rc::new(UH0::UH0_Cons(v77, v78));
                                    let v80: Rc<UH0> = Rc::new(UH0::UH0_Cons(v76, v79));
                                    let v81: Rc<UH0> = Rc::new(UH0::UH0_Cons(v75, v80));
                                    let v82: bool = named_170(v0.clone(), v81);
                                    if v82 {
                                        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("48,1632,1776,1984,2406,2534,2662,2790,2918,3046,3174,3302,3430,3558,3664,3792,3872,4160,4240,6112,6160,6470,6608,6784,6800,6992,7088,7232,7248,42528,43216,43264,43472,43504,43600,44016,65296,66720,68912,68928,69734,69872,69942,70096,70384,70736,70864,71248,71360,71376,71472,71904,72016,72688,72784,73040,73120,73552,90416,92768,92864,93008,93552,118000,120782,123200,123632,124144,124401,125264,130032"); } LIT.with(|lit| lit.clone()) };
                                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("57,1641,1785,1993,2415,2543,2671,2799,2927,3055,3183,3311,3439,3567,3673,3801,3881,4169,4249,6121,6169,6479,6617,6793,6809,7001,7097,7241,7257,42537,43225,43273,43481,43513,43609,44025,65305,66729,68921,68937,69743,69881,69951,70105,70393,70745,70873,71257,71369,71395,71481,71913,72025,72697,72793,73049,73129,73561,90425,92777,92873,93017,93561,118009,120831,123209,123641,124153,124410,125273,130041"); } LIT.with(|lit| lit.clone()) };
                                        US21::US21_Some(v83, v84)
                                    } else {
                                        let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nl"); } LIT.with(|lit| lit.clone()) };
                                        let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("letternumber"); } LIT.with(|lit| lit.clone()) };
                                        let v88: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                        let v89: Rc<UH0> = Rc::new(UH0::UH0_Cons(v87, v88));
                                        let v90: Rc<UH0> = Rc::new(UH0::UH0_Cons(v86, v89));
                                        let v91: bool = named_170(v0.clone(), v90);
                                        if v91 {
                                            let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("5870,8544,8581,12295,12321,12344,42726,65856,66369,66378,66513,74752"); } LIT.with(|lit| lit.clone()) };
                                            let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("5872,8578,8584,12295,12329,12346,42735,65908,66369,66378,66517,74862"); } LIT.with(|lit| lit.clone()) };
                                            US21::US21_Some(v92, v93)
                                        } else {
                                            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no"); } LIT.with(|lit| lit.clone()) };
                                            let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("othernumber"); } LIT.with(|lit| lit.clone()) };
                                            let v97: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                            let v98: Rc<UH0> = Rc::new(UH0::UH0_Cons(v96, v97));
                                            let v99: Rc<UH0> = Rc::new(UH0::UH0_Cons(v95, v98));
                                            let v100: bool = named_170(v0.clone(), v99);
                                            if v100 {
                                                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("178,185,188,2548,2930,3056,3192,3416,3440,3882,4969,6128,6618,8304,8308,8320,8528,8585,9312,9450,10102,11517,12690,12832,12872,12881,12928,12977,43056,65799,65909,65930,66273,66336,67672,67705,67751,67835,67862,68028,68032,68050,68160,68221,68253,68331,68440,68472,68521,68858,69216,69405,69457,69573,69714,70113,71482,71914,72794,73664,93019,93824,119488,119520,119648,125127,126065,126125,126129,126209,126255,127232"); } LIT.with(|lit| lit.clone()) };
                                                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("179,185,190,2553,2935,3058,3198,3422,3448,3891,4988,6137,6618,8304,8313,8329,8543,8585,9371,9471,10131,11517,12693,12841,12879,12895,12937,12991,43061,65843,65912,65931,66299,66339,67679,67711,67759,67839,67867,68029,68047,68095,68168,68222,68255,68335,68447,68479,68527,68863,69246,69414,69460,69579,69733,70132,71483,71922,72812,73684,93025,93846,119507,119539,119672,125135,126123,126127,126132,126253,126269,127244"); } LIT.with(|lit| lit.clone()) };
                                                US21::US21_Some(v101, v102)
                                            } else {
                                                let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("p"); } LIT.with(|lit| lit.clone()) };
                                                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("punctuation"); } LIT.with(|lit| lit.clone()) };
                                                let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("punct"); } LIT.with(|lit| lit.clone()) };
                                                let v107: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                let v108: Rc<UH0> = Rc::new(UH0::UH0_Cons(v106, v107));
                                                let v109: Rc<UH0> = Rc::new(UH0::UH0_Cons(v105, v108));
                                                let v110: Rc<UH0> = Rc::new(UH0::UH0_Cons(v104, v109));
                                                let v111: bool = named_170(v0.clone(), v110);
                                                if v111 {
                                                    let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("33,37,44,58,63,91,95,123,125,161,167,171,182,187,191,894,903,1370,1417,1470,1472,1475,1478,1523,1545,1548,1563,1565,1642,1748,1792,2039,2096,2142,2404,2416,2557,2678,2800,3191,3204,3572,3663,3674,3844,3860,3898,3973,4048,4057,4170,4347,4960,5120,5742,5787,5867,5941,6100,6104,6144,6468,6686,6816,6824,6990,7002,7037,7164,7227,7294,7360,7379,8208,8240,8261,8275,8317,8333,8968,9001,10088,10181,10214,10627,10712,10748,11513,11518,11632,11776,11824,11858,12289,12296,12308,12336,12349,12448,12539,42238,42509,42611,42622,42738,43124,43214,43256,43260,43310,43359,43457,43486,43612,43742,43760,44011,64830,65040,65072,65108,65123,65128,65130,65281,65285,65292,65306,65311,65339,65343,65371,65373,65375,65792,66463,66512,66927,67671,67871,67903,68176,68223,68336,68409,68505,68974,69293,69461,69510,69703,69819,69822,69952,70004,70085,70093,70107,70109,70200,70313,70612,70615,70731,70746,70749,70854,71105,71233,71264,71353,71484,71739,72004,72162,72255,72346,72350,72448,72673,72769,72816,73463,73539,73727,74864,77809,92782,92917,92983,92996,93549,93847,94178,113823,121479,124415,125278"); } LIT.with(|lit| lit.clone()) };
                                                    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("35,42,47,59,64,93,95,123,125,161,167,171,183,187,191,894,903,1375,1418,1470,1472,1475,1478,1524,1546,1549,1563,1567,1645,1748,1805,2041,2110,2142,2405,2416,2557,2678,2800,3191,3204,3572,3663,3675,3858,3860,3901,3973,4052,4058,4175,4347,4968,5120,5742,5788,5869,5942,6102,6106,6154,6469,6687,6822,6829,6991,7008,7039,7167,7231,7295,7367,7379,8231,8259,8273,8286,8318,8334,8971,9002,10101,10182,10223,10648,10715,10749,11516,11519,11632,11822,11855,11869,12291,12305,12319,12336,12349,12448,12539,42239,42511,42611,42622,42743,43127,43215,43258,43260,43311,43359,43469,43487,43615,43743,43761,44011,64831,65049,65106,65121,65123,65128,65131,65283,65290,65295,65307,65312,65341,65343,65371,65373,65381,65794,66463,66512,66927,67671,67871,67903,68184,68223,68342,68415,68508,68974,69293,69465,69513,69709,69820,69825,69955,70005,70088,70093,70107,70111,70205,70313,70613,70616,70735,70747,70749,70854,71127,71235,71276,71353,71486,71739,72006,72162,72262,72348,72354,72457,72673,72773,72817,73464,73551,73727,74868,77810,92783,92917,92987,92996,93551,93850,94178,113823,121483,124415,125279"); } LIT.with(|lit| lit.clone()) };
                                                    US21::US21_Some(v112, v113)
                                                } else {
                                                    let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                                                    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("symbol"); } LIT.with(|lit| lit.clone()) };
                                                    let v117: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    let v118: Rc<UH0> = Rc::new(UH0::UH0_Cons(v116, v117));
                                                    let v119: Rc<UH0> = Rc::new(UH0::UH0_Cons(v115, v118));
                                                    let v120: bool = named_170(v0.clone(), v119);
                                                    if v120 {
                                                        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("36,43,60,94,96,124,126,162,168,172,174,180,184,215,247,706,722,741,749,751,885,900,1014,1154,1421,1542,1547,1550,1758,1769,1789,2038,2046,2184,2546,2554,2801,2928,3059,3199,3407,3449,3647,3841,3859,3861,3866,3892,3894,3896,4030,4039,4046,4053,4254,5008,5741,6107,6464,6622,7009,7028,8125,8127,8141,8157,8173,8189,8260,8274,8314,8330,8352,8448,8451,8456,8468,8470,8478,8485,8487,8489,8494,8506,8512,8522,8527,8586,8592,8972,9003,9280,9372,9472,10132,10183,10224,10649,10716,10750,11126,11159,11493,11856,11904,11931,12032,12272,12292,12306,12320,12342,12350,12443,12688,12694,12736,12783,12800,12842,12880,12896,12938,12992,19904,42128,42752,42784,42889,43048,43062,43639,43867,43882,64297,64434,64832,64975,65020,65122,65124,65129,65284,65291,65308,65342,65344,65372,65374,65504,65512,65532,65847,65913,65932,65936,65952,66000,67703,68296,69006,71487,73685,92988,92997,113820,117760,118016,118608,118784,119040,119081,119146,119171,119180,119214,119296,119365,119552,120513,120539,120571,120597,120629,120655,120687,120713,120745,120771,120832,121399,121453,121462,121477,123215,123647,126124,126128,126254,126704,126976,127024,127136,127153,127169,127185,127245,127462,127504,127552,127568,127584,127744,128732,128752,128768,128891,128992,129008,129024,129040,129104,129120,129168,129200,129216,129280,129632,129648,129664,129679,129742,129759,129776,129792,129940"); } LIT.with(|lit| lit.clone()) };
                                                        let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("36,43,62,94,96,124,126,166,169,172,177,180,184,215,247,709,735,747,749,767,885,901,1014,1154,1423,1544,1547,1551,1758,1769,1790,2038,2047,2184,2547,2555,2801,2928,3066,3199,3407,3449,3647,3843,3859,3863,3871,3892,3894,3896,4037,4044,4047,4056,4255,5017,5741,6107,6464,6655,7018,7036,8125,8129,8143,8159,8175,8190,8260,8274,8316,8332,8384,8449,8454,8457,8468,8472,8483,8485,8487,8489,8494,8507,8516,8525,8527,8587,8967,9000,9257,9290,9449,10087,10180,10213,10626,10711,10747,11123,11157,11263,11498,11857,11929,12019,12245,12287,12292,12307,12320,12343,12351,12444,12689,12703,12773,12783,12830,12871,12880,12927,12976,13311,19967,42182,42774,42785,42890,43051,43065,43641,43867,43883,64297,64450,64847,64975,65023,65122,65126,65129,65284,65291,65310,65342,65344,65372,65374,65510,65518,65533,65855,65929,65934,65948,65952,66044,67704,68296,69007,71487,73713,92991,92997,113820,117999,118451,118723,119029,119078,119140,119148,119172,119209,119274,119361,119365,119638,120513,120539,120571,120597,120629,120655,120687,120713,120745,120771,121343,121402,121460,121475,121478,123215,123647,126124,126128,126254,126705,127019,127123,127150,127167,127183,127221,127405,127490,127547,127560,127569,127589,128727,128748,128764,128886,128985,129003,129008,129035,129095,129113,129159,129197,129211,129217,129619,129645,129660,129673,129734,129756,129769,129784,129938,130031"); } LIT.with(|lit| lit.clone()) };
                                                        US21::US21_Some(v121, v122)
                                                    } else {
                                                        let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("z"); } LIT.with(|lit| lit.clone()) };
                                                        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("separator"); } LIT.with(|lit| lit.clone()) };
                                                        let v126: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                        let v127: Rc<UH0> = Rc::new(UH0::UH0_Cons(v125, v126));
                                                        let v128: Rc<UH0> = Rc::new(UH0::UH0_Cons(v124, v127));
                                                        let v129: bool = named_170(v0.clone(), v128);
                                                        if v129 {
                                                            let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("32,160,5760,8192,8232,8239,8287,12288"); } LIT.with(|lit| lit.clone()) };
                                                            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("32,160,5760,8202,8233,8239,8287,12288"); } LIT.with(|lit| lit.clone()) };
                                                            US21::US21_Some(v130, v131)
                                                        } else {
                                                            let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cc"); } LIT.with(|lit| lit.clone()) };
                                                            let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("control"); } LIT.with(|lit| lit.clone()) };
                                                            let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cntrl"); } LIT.with(|lit| lit.clone()) };
                                                            let v136: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                            let v137: Rc<UH0> = Rc::new(UH0::UH0_Cons(v135, v136));
                                                            let v138: Rc<UH0> = Rc::new(UH0::UH0_Cons(v134, v137));
                                                            let v139: Rc<UH0> = Rc::new(UH0::UH0_Cons(v133, v138));
                                                            let v140: bool = named_170(v0, v139);
                                                            if v140 {
                                                                let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0,127"); } LIT.with(|lit| lit.clone()) };
                                                                let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("31,159"); } LIT.with(|lit| lit.clone()) };
                                                                US21::US21_Some(v141, v142)
                                                            } else {
                                                                US21::US21_None
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn either_table_168(mut v0: Rc<UH6>) -> US21 {
    let mut v1: US21 = category_table_173(v0.clone());
    match v1 {
        US21::US21_None => {
            script_table_169(v0)
        }
        US21::US21_Some(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            US21::US21_Some(v2, v3)
        }
    }
}
fn prefixed_table_174(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> US21 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gc"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("generalcategory"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v4));
    let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v5));
    let v7: bool = named_170(v0.clone(), v6);
    if v7 {
        category_table_173(v1)
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sc"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("script"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v11));
        let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v12));
        let v14: bool = named_170(v0, v13);
        if v14 {
            script_table_169(v1)
        } else {
            US21::US21_None
        }
    }
}
fn property_set_163(mut v0: Rc<UH6>) -> US19 {
    let mut v1: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    let v2: US18 = split_at_equals_164(v0.clone(), v1);
    let v11: US21 = match v2 {
        US18::US18_None => {
            let v8: Rc<UH6> = loose_key_165(v0);
            either_table_168(v8)
        }
        US18::US18_Some(v3, v4) => {
            let v3: Rc<UH6> = v3;
            let v4: Rc<UH6> = v4;
            let mut v5: Rc<UH6> = loose_key_165(v3);
            let v6: Rc<UH6> = loose_key_165(v4);
            prefixed_table_174(v5, v6)
        }
    };
    match v11 {
        US21::US21_None => {
            US19::US19_None
        }
        US21::US21_Some(v12, v13) => {
            let v12: Rc<str> = v12;
            let v13: Rc<str> = v13;
            let v14: US4 = table_set_75(v12, v13);
            US19::US19_Some(v14)
        }
    }
}
fn named_property_162(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: Rc<UH6>, mut v7: Rc<UH6>) -> US16 {
    if v0 {
        let v8: US2 = US2::US2_CaseFoldBeyondAscii;
        US16::US16_EscStopped(v8)
    } else {
        let v10: US19 = property_set_163(v6);
        match v10 {
            US19::US19_None => {
                let v15: US2 = US2::US2_UnicodeTableMissing;
                US16::US16_EscStopped(v15)
            }
            US19::US19_Some(v11) => {
                let v11: US4 = v11;
                let v13: US4 = if v5 {
                    complement_62(v11)
                } else {
                    v11
                };
                US16::US16_EscSet(v13, v7)
            }
        }
    }
}
fn braced_property_161(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: Rc<UH6>, mut v7: Rc<UH6>) -> US16 {
    loop {
        match Rc::unwrap_or_clone(v6) {
            UH6::UH6_Cons(v8, v9) => {
                let v8: u32 = v8;
                let mut v9: Rc<UH6> = v9;
                let v10: bool = v8 == 125u32;
                if v10 {
                    let v11: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                    let v12: Rc<UH6> = method147(v7, v11);
                    return named_property_162(v0, v1, v2, v3, v4, v5, v12, v9);
                } else {
                    let v14: Rc<UH6> = Rc::new(UH6::UH6_Cons(v8, v7));
                    (v6, v7) = (v9, v14);
                    continue;
                }
            }
            UH6::UH6_Nil => {
                let v17: US2 = US2::US2_EscapeUnknown;
                return US16::US16_EscStopped(v17);
            }
        }
    }
}
fn property_escape_160(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: Rc<UH6>) -> US16 {
    match Rc::unwrap_or_clone(v6) {
        UH6::UH6_Cons(v7, v8) => {
            let v7: u32 = v7;
            let v8: Rc<UH6> = v8;
            let mut v9: bool = v7 == 123u32;
            if v9 {
                let v10: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                braced_property_161(v0, v1, v2, v3, v4, v5, v8, v10)
            } else {
                let v12: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                let v13: Rc<UH6> = Rc::new(UH6::UH6_Cons(v7, v12));
                named_property_162(v0, v1, v2, v3, v4, v5, v13, v8)
            }
        }
        UH6::UH6_Nil => {
            let v16: US2 = US2::US2_EscapeUnknown;
            US16::US16_EscStopped(v16)
        }
    }
}
fn scalar_or_refused_178(mut v0: u32, mut v1: Rc<UH6>) -> US16 {
    let v2: bool = v0 > 1114111u32;
    let v6: bool = if v2 {
        true
    } else {
        let v3: bool = 55296u32 <= v0;
        if v3 {
            let v4: bool = v0 <= 57343u32;
            v4
        } else {
            false
        }
    };
    if v6 {
        let v7: US2 = US2::US2_HexInvalid;
        US16::US16_EscStopped(v7)
    } else {
        US16::US16_EscChar(v0, v1)
    }
}
fn hex_digit_179(mut v0: u32) -> i32 {
    let mut v1: bool = 48u32 <= v0;
    let v3: bool = if v1 {
        let v2: bool = v0 <= 57u32;
        v2
    } else {
        false
    };
    if v3 {
        let v4: u32 = v0.wrapping_sub(48u32);
        let mut v5: i32 = (v4 as i32);
        v5
    } else {
        let v6: bool = 97u32 <= v0;
        let v8: bool = if v6 {
            let v7: bool = v0 <= 102u32;
            v7
        } else {
            false
        };
        if v8 {
            let mut v9: u32 = v0.wrapping_sub(87u32);
            let v10: i32 = (v9 as i32);
            v10
        } else {
            let v11: bool = 65u32 <= v0;
            let v13: bool = if v11 {
                let v12: bool = v0 <= 70u32;
                v12
            } else {
                false
            };
            if v13 {
                let v14: u32 = v0.wrapping_sub(55u32);
                let v15: i32 = (v14 as i32);
                v15
            } else {
                -1i32
            }
        }
    }
}
fn hex_braced_177(mut v0: Rc<UH6>, mut v1: i32, mut v2: u32) -> US16 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v3, v4) => {
                let v3: u32 = v3;
                let v4: Rc<UH6> = v4;
                let mut v5: bool = v3 == 125u32;
                let v7: bool = if v5 {
                    let v6: bool = v1 > 0i32;
                    v6
                } else {
                    false
                };
                if v7 {
                    return scalar_or_refused_178(v2, v4);
                } else {
                    let mut v9: i32 = hex_digit_179(v3);
                    let v10: bool = v9 < 0i32;
                    let v12: bool = if v10 {
                        true
                    } else {
                        let v11: bool = v1 >= 8i32;
                        v11
                    };
                    if v12 {
                        let v13: US2 = US2::US2_HexInvalid;
                        return US16::US16_EscStopped(v13);
                    } else {
                        let v15: i32 = v1.wrapping_add(1i32);
                        let v16: u32 = v2.wrapping_mul(16u32);
                        let v17: i32 = hex_digit_179(v3);
                        let v18: u32 = (v17 as u32);
                        let v19: u32 = v16.wrapping_add(v18);
                        (v0, v1, v2) = (v4, v15, v19);
                        continue;
                    }
                }
            }
            UH6::UH6_Nil => {
                let v23: US2 = US2::US2_HexInvalid;
                return US16::US16_EscStopped(v23);
            }
        }
    }
}
fn hex_fixed_180(mut v0: Rc<UH6>, mut v1: i32, mut v2: u32) -> US16 {
    loop {
        let v3: bool = v1 <= 0i32;
        if v3 {
            return scalar_or_refused_178(v2, v0);
        } else {
            match Rc::unwrap_or_clone(v0) {
                UH6::UH6_Cons(v5, v6) => {
                    let mut v5: u32 = v5;
                    let v6: Rc<UH6> = v6;
                    let v7: i32 = hex_digit_179(v5);
                    let v8: bool = v7 < 0i32;
                    if v8 {
                        let mut v9: US2 = US2::US2_HexInvalid;
                        return US16::US16_EscStopped(v9);
                    } else {
                        let v11: i32 = v1.wrapping_sub(1i32);
                        let v12: u32 = v2.wrapping_mul(16u32);
                        let v13: i32 = hex_digit_179(v5);
                        let v14: u32 = (v13 as u32);
                        let v15: u32 = v12.wrapping_add(v14);
                        (v0, v1, v2) = (v6, v11, v15);
                        continue;
                    }
                }
                UH6::UH6_Nil => {
                    let v18: US2 = US2::US2_HexInvalid;
                    return US16::US16_EscStopped(v18);
                }
            }
        }
    }
}
fn hex_escape_176(mut v0: i32, mut v1: Rc<UH6>) -> US16 {
    let v2: u32 = 123u32;
    let v3: bool = heads_20(v2, v1.clone());
    if v3 {
        let v4: Rc<UH6> = after_one_44(v1);
        let mut v5: i32 = 0i32;
        let v6: u32 = 0u32;
        hex_braced_177(v4, v5, v6)
    } else {
        let v8: u32 = 0u32;
        hex_fixed_180(v1, v0, v8)
    }
}
fn control_escape_182(mut v0: u32) -> u32 {
    let mut v1: bool = v0 == 97u32;
    if v1 {
        7u32
    } else {
        let v2: bool = v0 == 102u32;
        if v2 {
            12u32
        } else {
            let v3: bool = v0 == 116u32;
            if v3 {
                9u32
            } else {
                let v4: bool = v0 == 110u32;
                if v4 {
                    10u32
                } else {
                    let mut v5: bool = v0 == 114u32;
                    if v5 {
                        13u32
                    } else {
                        let v6: bool = v0 == 118u32;
                        if v6 {
                            11u32
                        } else {
                            0u32
                        }
                    }
                }
            }
        }
    }
}
fn literal_escape_181(mut v0: u32, mut v1: Rc<UH6>) -> US16 {
    let v2: u32 = control_escape_182(v0);
    let v3: bool = v2 > 0u32;
    if v3 {
        let v4: u32 = control_escape_182(v0);
        US16::US16_EscChar(v4, v1)
    } else {
        let v6: bool = v0 == 60u32;
        let v8: bool = if v6 {
            true
        } else {
            let v7: bool = v0 == 62u32;
            v7
        };
        if v8 {
            let mut v9: US2 = US2::US2_WordBoundaryUnsupported;
            US16::US16_EscStopped(v9)
        } else {
            let v11: bool = v0 < 128u32;
            let v14: bool = if v11 {
                let v12: bool = is_ascii_alnum_120(v0);
                let v13: bool = v12 == false;
                v13
            } else {
                false
            };
            if v14 {
                US16::US16_EscChar(v0, v1)
            } else {
                let v16: bool = 49u32 <= v0;
                let v18: bool = if v16 {
                    let v17: bool = v0 <= 57u32;
                    v17
                } else {
                    false
                };
                let v20: bool = if v18 {
                    true
                } else {
                    let v19: bool = v0 == 107u32;
                    v19
                };
                if v20 {
                    let v21: US2 = US2::US2_BackreferenceUnsupported;
                    US16::US16_EscStopped(v21)
                } else {
                    let v23: bool = v0 == 98u32;
                    let v25: bool = if v23 {
                        true
                    } else {
                        let v24: bool = v0 == 66u32;
                        v24
                    };
                    if v25 {
                        let v26: US2 = US2::US2_WordBoundaryUnsupported;
                        US16::US16_EscStopped(v26)
                    } else {
                        let v28: bool = v0 == 112u32;
                        let v30: bool = if v28 {
                            true
                        } else {
                            let v29: bool = v0 == 80u32;
                            v29
                        };
                        if v30 {
                            let v31: US2 = US2::US2_UnicodeTableMissing;
                            US16::US16_EscStopped(v31)
                        } else {
                            let v33: US2 = US2::US2_EscapeUnknown;
                            US16::US16_EscStopped(v33)
                        }
                    }
                }
            }
        }
    }
}
fn hex_or_literal_175(mut v0: u32, mut v1: Rc<UH6>) -> US16 {
    let v2: bool = v0 == 120u32;
    if v2 {
        let v3: i32 = 2i32;
        hex_escape_176(v3, v1)
    } else {
        let mut v5: bool = v0 == 117u32;
        if v5 {
            let v6: i32 = 4i32;
            hex_escape_176(v6, v1)
        } else {
            let v8: bool = v0 == 85u32;
            if v8 {
                let mut v9: i32 = 8i32;
                hex_escape_176(v9, v1)
            } else {
                literal_escape_181(v0, v1)
            }
        }
    }
}
fn unicode_decimal_183() -> US4 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("48,1632,1776,1984,2406,2534,2662,2790,2918,3046,3174,3302,3430,3558,3664,3792,3872,4160,4240,6112,6160,6470,6608,6784,6800,6992,7088,7232,7248,42528,43216,43264,43472,43504,43600,44016,65296,66720,68912,68928,69734,69872,69942,70096,70384,70736,70864,71248,71360,71376,71472,71904,72016,72688,72784,73040,73120,73552,90416,92768,92864,93008,93552,118000,120782,123200,123632,124144,124401,125264,130032"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("57,1641,1785,1993,2415,2543,2671,2799,2927,3055,3183,3311,3439,3567,3673,3801,3881,4169,4249,6121,6169,6479,6617,6793,6809,7001,7097,7241,7257,42537,43225,43273,43481,43513,43609,44025,65305,66729,68921,68937,69743,69881,69951,70105,70393,70745,70873,71257,71369,71395,71481,71913,72025,72697,72793,73049,73129,73561,90425,92777,92873,93017,93561,118009,120831,123209,123641,124153,124410,125273,130041"); } LIT.with(|lit| lit.clone()) };
    table_set_75(v0, v1)
}
fn escape_code_155(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: u32, mut v6: Rc<UH6>) -> US16 {
    let v7: bool = v5 == 100u32;
    let mut v9: bool = if v7 {
        true
    } else {
        let v8: bool = v5 == 68u32;
        v8
    };
    if v9 {
        let v10: u32 = 48u32;
        let v11: u32 = 57u32;
        let v12: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
        let v13: Rc<UH5> = Rc::new(UH5::UH5_Cons(v10, v11, v12));
        let v14: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
        let v15: Rc<UH5> = sorted_by_low_82(v13, v14);
        let v16: Rc<UH5> = merged_84(v15);
        let v17: US4 = US4::US4_Normal(v16);
        let v18: US4 = unicode_decimal_183();
        let v19: bool = v5 == 68u32;
        perl_set_156(v0, v1, v2, v3, v4, v17, v18, v19, v6)
    } else {
        let v21: bool = v5 == 119u32;
        let v23: bool = if v21 {
            true
        } else {
            let v22: bool = v5 == 87u32;
            v22
        };
        if v23 {
            let v24: US4 = ascii_word_86();
            let v25: US4 = unicode_word_74();
            let v26: bool = v5 == 87u32;
            perl_set_156(v0, v1, v2, v3, v4, v24, v25, v26, v6)
        } else {
            let v28: bool = v5 == 115u32;
            let v30: bool = if v28 {
                true
            } else {
                let v29: bool = v5 == 83u32;
                v29
            };
            if v30 {
                let v31: bool = v5 == 83u32;
                space_set_157(v0, v1, v2, v3, v4, v31, v6)
            } else {
                let v33: bool = v5 == 112u32;
                let v35: bool = if v33 {
                    true
                } else {
                    let v34: bool = v5 == 80u32;
                    v34
                };
                let v36: bool = v35 && v3;
                if v36 {
                    let v37: bool = v5 == 80u32;
                    property_escape_160(v0, v1, v2, v3, v4, v37, v6)
                } else {
                    hex_or_literal_175(v5, v6)
                }
            }
        }
    }
}
fn escape_after_154(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: &Rc<UH6>) -> US16 {
    match &**v5 {
        UH6::UH6_Cons(v6, v7) => {
            let v6: u32 = *v6;
            let v7: Rc<UH6> = v7.clone();
            escape_code_155(v0, v1, v2, v3, v4, v6, v7)
        }
        UH6::UH6_Nil => {
            let mut v9: US2 = US2::US2_EscapeUnknown;
            US16::US16_EscStopped(v9)
        }
    }
}
fn range_high_153(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: &Rc<UH6>) -> US20 {
    match &**v5 {
        UH6::UH6_Cons(v6, v7) => {
            let v6: u32 = *v6;
            let v7: Rc<UH6> = v7.clone();
            match Rc::unwrap_or_clone(v7) {
                UH6::UH6_Cons(v8, v9) => {
                    let v8: u32 = v8;
                    let mut v9: Rc<UH6> = v9;
                    let v10: bool = v6 != 45u32;
                    let v12: bool = if v10 {
                        true
                    } else {
                        let v11: bool = v8 == 93u32;
                        v11
                    };
                    let v14: bool = if v12 {
                        true
                    } else {
                        let v13: bool = v8 == 45u32;
                        v13
                    };
                    let v16: bool = if v14 {
                        true
                    } else {
                        let v15: bool = v8 == 91u32;
                        v15
                    };
                    if v16 {
                        US20::US20_None
                    } else {
                        let v18: bool = v8 == 92u32;
                        if v18 {
                            let v19: US16 = escape_after_154(v0, v1, v2, v3, v4, &v9);
                            match v19 {
                                US16::US16_EscChar(v20, v21) => {
                                    let v20: u32 = v20;
                                    let v21: Rc<UH6> = v21;
                                    US20::US20_Some(v20, v21)
                                }
                                _ => {
                                    US20::US20_None
                                }
                            }
                        } else {
                            US20::US20_Some(v8, v9)
                        }
                    }
                }
                _ => {
                    US20::US20_None
                }
            }
        }
        _ => {
            US20::US20_None
        }
    }
}
fn ranged_152(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: u32, mut v6: Rc<UH6>) -> US15 {
    let v7: US20 = range_high_153(v0, v1, v2, v3, v4, &v6);
    match v7 {
        US20::US20_None => {
            literal_class_125(v0, v1, v2, v3, v4, v5, v6)
        }
        US20::US20_Some(v8, v9) => {
            let v8: u32 = v8;
            let mut v9: Rc<UH6> = v9;
            let v10: bool = v5 <= v8;
            if v10 {
                let v11: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
                let v12: Rc<UH5> = Rc::new(UH5::UH5_Cons(v5, v8, v11));
                let v13: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
                let v14: Rc<UH5> = sorted_by_low_82(v12, v13);
                let v15: Rc<UH5> = merged_84(v14);
                let v16: US4 = US4::US4_Normal(v15);
                fold_class_126(v0, v1, v2, v3, v4, v16, v9)
            } else {
                let v18: US2 = US2::US2_CountInvalid;
                US15::US15_ClassStopped(v18)
            }
        }
    }
}
fn escaped_class_184(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US16) -> US15 {
    match v5 {
        US16::US16_EscChar(v6, v7) => {
            let v6: u32 = v6;
            let v7: Rc<UH6> = v7;
            literal_class_125(v0, v1, v2, v3, v4, v6, v7)
        }
        US16::US16_EscSet(v9, v10) => {
            let mut v9: US4 = v9;
            let v10: Rc<UH6> = v10;
            US15::US15_ClassGot(v9, v10)
        }
        US16::US16_EscStopped(v12) => {
            let v12: US2 = v12;
            US15::US15_ClassStopped(v12)
        }
    }
}
fn escaped_ranged_151(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US16) -> US15 {
    match &v5 {
        US16::US16_EscChar(v6, v7) => {
            let v6: u32 = *v6;
            let v7: Rc<UH6> = v7.clone();
            ranged_152(v0, v1, v2, v3, v4, v6, v7)
        }
        _ => {
            escaped_class_184(v0, v1, v2, v3, v4, v5)
        }
    }
}
fn class_item_143(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: u32, mut v6: Rc<UH6>) -> US15 {
    let v7: bool = v5 == 91u32;
    if v7 {
        class_nested_144(v0, v1, v2, v3, v4, v6)
    } else {
        let mut v9: bool = v5 == 92u32;
        if v9 {
            let v10: US16 = escape_after_154(v0, v1, v2, v3, v4, &v6);
            escaped_ranged_151(v0, v1, v2, v3, v4, v10)
        } else {
            ranged_152(v0, v1, v2, v3, v4, v5, v6)
        }
    }
}
enum Group139 {
    M139(bool, bool, bool, bool, bool, Rc<UH6>, US4, bool),
    M140(bool, bool, bool, bool, bool, u32, Rc<UH6>, US4),
}
fn group_139(mut st: Group139) -> US15 {
    loop {
        match st {
            Group139::M139(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7) => {
                let v8: Rc<UH6> = class_skip_141(v0, v1, v2, v3, v4, v5);
                match &*v8 {
                    UH6::UH6_Cons(v9, v10) => {
                        let mut v9: u32 = *v9;
                        let v10: Rc<UH6> = v10.clone();
                        let v11: bool = v9 == 93u32;
                        let v13: bool = if v11 {
                            let v12: bool = v7 == false;
                            v12
                        } else {
                            false
                        };
                        if v13 {
                            return US15::US15_ClassGot(v6, v8);
                        } else {
                            let v15: bool = starts_op_142(&v8);
                            if v15 {
                                return US15::US15_ClassGot(v6, v8);
                            } else {
                                st = Group139::M140(v0, v1, v2, v3, v4, v9, v10, v6);
                                continue;
                            }
                        }
                    }
                    UH6::UH6_Nil => {
                        let v20: US2 = US2::US2_ClassUnclosed;
                        return US15::US15_ClassStopped(v20);
                    }
                }
            }
            Group139::M140(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7) => {
                let v8: US15 = class_item_143(v0, v1, v2, v3, v4, v5, v6);
                match &v8 {
                    US15::US15_ClassGot(v9, v10) => {
                        let mut v9: US4 = v9.clone();
                        let v10: Rc<UH6> = v10.clone();
                        let v11: US4 = set_union_129(v7, v9);
                        let v12: bool = false;
                        st = Group139::M139(v0, v1, v2, v3, v4, v10, v11, v12);
                        continue;
                    }
                    _ => {
                        return v8;
                    }
                }
            }
        }
    }
}
fn class_item_then_140(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: u32, mut v6: Rc<UH6>, mut v7: US4) -> US15 {
    group_139(Group139::M140(v0, v1, v2, v3, v4, v5, v6, v7))
}
fn class_union_139(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH6>, mut v6: US4, mut v7: bool) -> US15 {
    group_139(Group139::M139(v0, v1, v2, v3, v4, v5, v6, v7))
}
fn minus_187(mut v0: US4, mut v1: US4) -> US4 {
    let v2: US4 = complement_62(v1);
    intersection_128(v0, v2)
}
fn class_combine_186(mut v0: u32, mut v1: US4, mut v2: US15) -> US15 {
    match &v2 {
        US15::US15_ClassGot(v3, v4) => {
            let v3: US4 = v3.clone();
            let v4: Rc<UH6> = v4.clone();
            let mut v5: bool = v0 == 38u32;
            let v13: US4 = if v5 {
                intersection_128(v1, v3)
            } else {
                let v7: bool = v0 == 45u32;
                if v7 {
                    minus_187(v1, v3)
                } else {
                    let mut v9: US4 = minus_187(v1.clone(), v3.clone());
                    let v10: US4 = minus_187(v3, v1);
                    set_union_129(v9, v10)
                }
            };
            US15::US15_ClassGot(v13, v4)
        }
        _ => {
            v2
        }
    }
}
fn class_ops_185(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: US15) -> US15 {
    loop {
        match &v5 {
            US15::US15_ClassGot(v6, v7) => {
                let v6: US4 = v6.clone();
                let v7: Rc<UH6> = v7.clone();
                match &*v7 {
                    UH6::UH6_Cons(v8, v9) => {
                        let v8: u32 = *v8;
                        let mut v9: Rc<UH6> = v9.clone();
                        match Rc::unwrap_or_clone(v9) {
                            UH6::UH6_Cons(_, v11) => {
                                let v11: Rc<UH6> = v11;
                                let v12: bool = starts_op_142(&v7);
                                if v12 {
                                    let v13: US4 = nothing_set_137();
                                    let v14: bool = true;
                                    let v15: US15 = class_union_139(v0, v1, v2, v3, v4, v11, v13, v14);
                                    let v16: US15 = class_combine_186(v8, v6, v15);
                                    v5 = v16;
                                    continue;
                                } else {
                                    return v5;
                                }
                            }
                            _ => {
                                return v5;
                            }
                        }
                    }
                    _ => {
                        return v5;
                    }
                }
            }
            _ => {
                return v5;
            }
        }
    }
}
fn class_open_138(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: Rc<UH6>) -> US15 {
    let v6: u32 = 94u32;
    let v7: bool = heads_20(v6, v5.clone());
    let mut v9: Rc<UH6> = if v7 {
        after_one_44(v5)
    } else {
        v5
    };
    let v10: US4 = nothing_set_137();
    let v11: bool = true;
    let v12: US15 = class_union_139(v0, v1, v2, v3, v4, v9, v10, v11);
    let v13: US15 = class_ops_185(v0, v1, v2, v3, v4, v12);
    match &v13 {
        US15::US15_ClassGot(v14, v15) => {
            let v14: US4 = v14.clone();
            let v15: Rc<UH6> = v15.clone();
            let v16: u32 = 93u32;
            let v17: bool = heads_20(v16, v15.clone());
            if v17 {
                let v19: US4 = if v7 {
                    complement_62(v14)
                } else {
                    v14
                };
                let v20: Rc<UH6> = after_one_44(v15);
                US15::US15_ClassGot(v19, v20)
            } else {
                let v22: US2 = US2::US2_ClassUnclosed;
                US15::US15_ClassStopped(v22)
            }
        }
        _ => {
            v13
        }
    }
}
fn dot_188(mut v0: bool, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool) -> Rc<UH4> {
    if v1 {
        let mut v5: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
        let v6: Rc<UH5> = Rc::new(UH5::UH5_Cons(0u32, 1114111u32, v5));
        let v7: US4 = US4::US4_Normal(v6);
        Rc::new(UH4::UH4_Set(v7))
    } else {
        let mut v9: u32 = 10u32;
        let v10: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
        let v11: Rc<UH5> = Rc::new(UH5::UH5_Cons(v9, v9, v10));
        let v12: US4 = US4::US4_Normal(v11);
        let v13: US4 = complement_62(v12);
        let v14: bool = is_empty_61(v13.clone());
        if v14 {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nothing); } CASE.with(|case| case.clone()) }
        } else {
            Rc::new(UH4::UH4_Set(v13))
        }
    }
}
fn rust_atom_116(mut v0: &Rc<UH6>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool) -> US3 {
    match &**v0 {
        UH6::UH6_Cons(v6, v7) => {
            let v6: u32 = *v6;
            let v7: Rc<UH6> = v7.clone();
            let v8: bool = v6 == 40u32;
            if v8 {
                rust_open_117(v7, v1, v2, v3, v4, v5)
            } else {
                let v10: bool = v6 == 91u32;
                if v10 {
                    let v11: US15 = class_open_138(v1, v2, v3, v4, v5, v7);
                    class_to_step_122(v1, v2, v3, v4, v5, v11)
                } else {
                    let v13: bool = v6 == 92u32;
                    if v13 {
                        let v14: US16 = escape_after_154(v1, v2, v3, v4, v5, &v7);
                        escape_step_123(v1, v2, v3, v4, v5, v14)
                    } else {
                        let v16: bool = v6 == 46u32;
                        if v16 {
                            let v17: Rc<UH4> = dot_188(v1, v2, v3, v4, v5);
                            US3::US3_Got(v17, v7, v1, v2, v3, v4, v5)
                        } else {
                            let v19: bool = is_quantifier_109(v6);
                            if v19 {
                                let v20: US2 = US2::US2_RepeatOfNothing;
                                US3::US3_Stopped(v20)
                            } else {
                                literal_step_124(v1, v2, v3, v4, v5, v6, v7)
                            }
                        }
                    }
                }
            }
        }
        UH6::UH6_Nil => {
            let v28: US2 = US2::US2_RepeatOfNothing;
            US3::US3_Stopped(v28)
        }
    }
}
enum Group45 {
    M45(Rc<UH6>, bool, bool, bool, bool, bool, bool, US8, Rc<UH4>, bool),
    M90(Rc<UH6>, bool, bool, bool, bool, bool, bool, US8, Rc<UH4>, bool),
    M91(bool, US8, Rc<UH4>, US3),
}
fn group_45(mut st: Group45) -> US3 {
    loop {
        match st {
            Group45::M45(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9) => {
                let v10: Rc<UH6> = skip_space_94(v1, v2, v3, v4, v5, v0);
                let v11: bool = rust_ends_branch_98(v10.clone());
                if v11 {
                    let v12: US8 = US8::US8_Unpinned;
                    return rust_close_46(v1, v2, v3, v4, v5, v6, v7, v12, v8, v10);
                } else {
                    let v14: US10 = anchor_of_99(v1, v2, v3, v4, v5, &v10);
                    match v14 {
                        US10::US10_EndAnchor(v26, v27) => {
                            let v26: US8 = v26;
                            let v27: Rc<UH6> = v27;
                            let v28: Rc<UH6> = skip_space_94(v1, v2, v3, v4, v5, v27);
                            let v30: bool = if v6 {
                                rust_ends_branch_98(v28.clone())
                            } else {
                                false
                            };
                            if v30 {
                                return rust_close_46(v1, v2, v3, v4, v5, v6, v7, v26, v8, v28);
                            } else {
                                let v32: US2 = US2::US2_AnchorInside;
                                return US3::US3_Stopped(v32);
                            }
                        }
                        US10::US10_NoAnchor => {
                            st = Group45::M90(v10, v1, v2, v3, v4, v5, v6, v7, v8, v9);
                            continue;
                        }
                        US10::US10_StartAnchor(v15, v16) => {
                            let v15: US8 = v15;
                            let v16: Rc<UH6> = v16;
                            match v7 {
                                US8::US8_Unpinned => {
                                    let v18: bool = if v6 {
                                        let v17: bool = v9 == false;
                                        v17
                                    } else {
                                        false
                                    };
                                    if v18 {
                                        st = Group45::M45(v16, v1, v2, v3, v4, v5, v6, v15, v8, v9);
                                        continue;
                                    } else {
                                        let v20: US2 = US2::US2_AnchorInside;
                                        return US3::US3_Stopped(v20);
                                    }
                                }
                                _ => {
                                    let v23: US2 = US2::US2_AnchorInside;
                                    return US3::US3_Stopped(v23);
                                }
                            }
                        }
                        US10::US10_WordAnchor(v35) => {
                            let v35: Rc<UH6> = v35;
                            let v36: Rc<UH6> = skip_space_94(v1, v2, v3, v4, v5, v35.clone());
                            let v38: bool = if v6 {
                                let v37: bool = v9 == false;
                                v37
                            } else {
                                false
                            };
                            let v40: bool = if v38 {
                                is_unpinned_100(v7.clone())
                            } else {
                                false
                            };
                            let v43: bool = if v40 {
                                let v41: bool = rust_ends_branch_98(v36.clone());
                                let v42: bool = v41 == false;
                                v42
                            } else {
                                false
                            };
                            if v43 {
                                let v44: US8 = US8::US8_WordEdge;
                                st = Group45::M45(v35, v1, v2, v3, v4, v5, v6, v44, v8, v9);
                                continue;
                            } else {
                                let v46: bool = v6 && v9;
                                let v48: bool = if v46 {
                                    rust_ends_branch_98(v36.clone())
                                } else {
                                    false
                                };
                                if v48 {
                                    let v49: US8 = US8::US8_WordEdge;
                                    return rust_close_46(v1, v2, v3, v4, v5, v6, v7, v49, v8, v36);
                                } else {
                                    let v53: bool = if v46 {
                                        let v51: bool = nullable_48(v8.clone());
                                        let v52: bool = v51 == false;
                                        v52
                                    } else {
                                        false
                                    };
                                    if v53 {
                                        let v54: US8 = US8::US8_PinnedText;
                                        let v55: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
                                        let v56: bool = false;
                                        let v57: US3 = rust_branch_45(v35, v1, v2, v3, v4, v5, v6, v54, v55, v56);
                                        return rust_mid_boundary_92(v1, v2, v3, v4, v5, v7, v8, v57);
                                    } else {
                                        let v59: US2 = US2::US2_WordBoundaryUnsupported;
                                        return US3::US3_Stopped(v59);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Group45::M90(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9) => {
                let v10: US11 = flags_at_101(v1, v2, v3, v4, v5, v0.clone());
                match v10 {
                    US11::US11_FlagsBad => {
                        let v29: US2 = US2::US2_FlagUnknown;
                        return US3::US3_Stopped(v29);
                    }
                    US11::US11_FlagsScoped(v18, v19, v20, v21, v22, v23) => {
                        let v18: bool = v18;
                        let v19: bool = v19;
                        let v20: bool = v20;
                        let v21: bool = v21;
                        let v22: bool = v22;
                        let v23: Rc<UH6> = v23;
                        let v24: bool = false;
                        let v25: US3 = rust_alternation_25(v23, v18, v19, v20, v21, v22, v24);
                        let v26: US3 = rust_group_105(v1, v2, v3, v4, v5, v25);
                        let v27: US3 = rust_quantified_106(v26);
                        st = Group45::M91(v6, v7, v8, v27);
                        continue;
                    }
                    US11::US11_FlagsSet(v11, v12, v13, v14, v15, v16) => {
                        let v11: bool = v11;
                        let v12: bool = v12;
                        let v13: bool = v13;
                        let v14: bool = v14;
                        let v15: bool = v15;
                        let v16: Rc<UH6> = v16;
                        st = Group45::M45(v16, v11, v12, v13, v14, v15, v6, v7, v8, v9);
                        continue;
                    }
                    US11::US11_NotFlags => {
                        let v31: US3 = rust_atom_116(&v0, v1, v2, v3, v4, v5);
                        let v32: US3 = rust_quantified_106(v31);
                        st = Group45::M91(v6, v7, v8, v32);
                        continue;
                    }
                }
            }
            Group45::M91(mut v0, mut v1, mut v2, mut v3) => {
                match &v3 {
                    US3::US3_Got(v4, v5, v6, v7, v8, v9, v10) => {
                        let v4: Rc<UH4> = v4.clone();
                        let mut v5: Rc<UH6> = v5.clone();
                        let v6: bool = *v6;
                        let v7: bool = *v7;
                        let v8: bool = *v8;
                        let mut v9: bool = *v9;
                        let v10: bool = *v10;
                        let v11: Rc<UH4> = seq_67(v2, v4);
                        let v12: bool = true;
                        st = Group45::M45(v5, v6, v7, v8, v9, v10, v0, v1, v11, v12);
                        continue;
                    }
                    _ => {
                        return v3;
                    }
                }
            }
        }
    }
}
fn rust_branch_piece_91(mut v0: bool, mut v1: US8, mut v2: Rc<UH4>, mut v3: US3) -> US3 {
    group_45(Group45::M91(v0, v1, v2, v3))
}
fn rust_branch_flags_90(mut v0: Rc<UH6>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: bool, mut v7: US8, mut v8: Rc<UH4>, mut v9: bool) -> US3 {
    group_45(Group45::M90(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
}
fn rust_branch_45(mut v0: Rc<UH6>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: bool, mut v7: US8, mut v8: Rc<UH4>, mut v9: bool) -> US3 {
    group_45(Group45::M45(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
}
fn rust_alternation_25(mut v0: Rc<UH6>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: bool) -> US3 {
    let v7: US8 = US8::US8_Unpinned;
    let v8: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Epsilon); } CASE.with(|case| case.clone()) };
    let mut v9: bool = false;
    let v10: US3 = rust_branch_45(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9);
    rust_after_branch_26(v6, v10)
}
fn parse_rust_18(mut v0: Rc<str>) -> US6 {
    let mut v1: Rc<UH6> = rust_codes_21(v0);
    let (mut v2, mut v3, mut v4, mut v5, mut v6): (bool, bool, bool, bool, bool) = default_flags_24();
    let v7: bool = true;
    let v8: US3 = rust_alternation_25(v1, v2, v3, v4, v5, v6, v7);
    rust_parsed_of_19(v8)
}
fn rust_refusal_of_17(mut v0: Rc<str>) -> US1 {
    let mut v1: US6 = parse_rust_18(v0);
    match v1 {
        US6::US6_RustParsed(_) => {
            US1::US1_None
        }
        US6::US6_RustRefused(v4) => {
            let v4: US2 = v4;
            US1::US1_Some(v4)
        }
    }
}
fn refusal_name_189(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_AnchorInside => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("anchor inside a group or mid-branch (not yet exact)"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US2::US2_BackreferenceUnsupported => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backreference (the regex crate has none)"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US2::US2_CaseFoldBeyondAscii => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("case folding beyond ASCII (not yet exact)"); } LIT.with(|lit| lit.clone()) };
            v12
        }
        US2::US2_ClassUnclosed => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unclosed class"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US2::US2_CloseWithoutOpen => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("close without open"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_CountInvalid => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid count"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US2::US2_EscapeUnknown => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown escape"); } LIT.with(|lit| lit.clone()) };
            v13
        }
        US2::US2_FlagUnknown => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown flag"); } LIT.with(|lit| lit.clone()) };
            v14
        }
        US2::US2_GroupUnclosed => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unclosed group"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_HexInvalid => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid hex scalar"); } LIT.with(|lit| lit.clone()) };
            v15
        }
        US2::US2_LookaroundUnsupported => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lookaround (the regex crate has none)"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US2::US2_RepeatOfNothing => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repetition of nothing"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_RepeatStacked => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stacked repetition"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_UnicodeTableMissing => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unicode property not tabled"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US2::US2_WordBoundaryUnsupported => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("word boundary inside a group or after a part that can be empty (not yet exact)"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn outcome_text_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US1 = rust_refusal_of_17(v0);
    match v1 {
        US1::US1_None => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("parsed exactly; agrees with the regex crate on the seeded corpus"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US1::US1_Some(v3) => {
            let v3: US2 = v3;
            let v4: Rc<str> = refusal_name_189(v3);
            let mut v5: Rc<str> = string_concat(&["refused: ", &*v4]);
            v5
        }
    }
}
fn outcome_row_15(mut v0: Rc<str>) -> Rc<UH2> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("code"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: US0 = escape_5(v0.clone());
    let v4: US0 = element_10(v1, v2, v3);
    let mut v5: Rc<str> = outcome_text_16(v0);
    let v6: US0 = escape_5(v5);
    let v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH2> = Rc::new(UH2::UH2_Cons(v6, v7));
    Rc::new(UH2::UH2_Cons(v4, v8))
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH3> = method14(v3, v1);
            let mut v5: Rc<UH2> = outcome_row_15(v2);
            Rc::new(UH3::UH3_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method192(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH2>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v2) {
        UH2::UH2_Cons(v4, v5) => {
            let v4: US0 = v4;
            let mut v5: Rc<UH2> = v5;
            let v6: Rc<UH2> = method192(v0.clone(), v1.clone(), v5, v3);
            let v7: US0 = element_10(v0, v1, v4);
            Rc::new(UH2::UH2_Cons(v7, v6))
        }
        UH2::UH2_Nil => {
            v3
        }
    }
}
fn joined_194(mut v0: US0, mut v1: US0) -> US0 {
    let v2: Rc<str> = markup_text_13(v0);
    let v3: Rc<str> = markup_text_13(v1);
    let v4: Rc<str> = string_concat(&[&*v2, &*v3]);
    US0::US0_Markup(v4)
}
fn concat_193(mut v0: Rc<UH2>) -> US0 {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v1, v2) => {
            let mut v1: US0 = v1;
            let v2: Rc<UH2> = v2;
            let v3: US0 = concat_193(v2);
            joined_194(v1, v3)
        }
        UH2::UH2_Nil => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            US0::US0_Markup(v5)
        }
    }
}
fn row_of_191(mut v0: Rc<str>, mut v1: Rc<UH2>) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tr"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH2> = method192(v0, v4, v1, v5);
    let v7: US0 = concat_193(v6);
    element_10(v2, v3, v7)
}
fn method195(mut v0: Rc<str>, mut v1: Rc<UH3>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v1) {
        UH3::UH3_Cons(v3, v4) => {
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH3> = v4;
            let mut v5: Rc<UH2> = method195(v0.clone(), v4, v2);
            let v6: US0 = row_of_191(v0, v3);
            Rc::new(UH2::UH2_Cons(v6, v5))
        }
        UH3::UH3_Nil => {
            v2
        }
    }
}
fn table_190(mut v0: Rc<UH2>, mut v1: Rc<UH3>) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("th"); } LIT.with(|lit| lit.clone()) };
    let mut v5: US0 = row_of_191(v4, v0);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("td"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH2> = method195(v6, v1, v7);
    let mut v9: US0 = concat_193(v8);
    let v10: US0 = joined_194(v5, v9);
    element_10(v2, v3, v10)
}
fn closure0() -> Rc<dyn Fn(i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<str>> = Rc::new(move |mut v0: i32| -> Rc<str> {
        let mut v1: Rc<UH0> = agreement_patterns_0();
        let v2: Rc<UH0> = refusal_examples_1();
        let v3: Rc<UH0> = method2(v1, v2);
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v5: Rc<UH0> = first_n_3(v0, v3, v4);
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("outcome"); } LIT.with(|lit| lit.clone()) };
        let v7: US0 = escape_5(v6);
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pattern"); } LIT.with(|lit| lit.clone()) };
        let mut v9: US0 = escape_5(v8);
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h2"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rust syntax conformance"); } LIT.with(|lit| lit.clone()) };
        let v13: US0 = escape_5(v12);
        let v14: US0 = element_10(v10, v11, v13);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("p"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rust regex syntax parsed by regex_rust (lib) into the derivative engine; every accepted pattern below is checked against the regex crate on a seeded corpus in the regex_rust_agreement Cells test."); } LIT.with(|lit| lit.clone()) };
        let v18: US0 = escape_5(v17);
        let v19: US0 = element_10(v15, v16, v18);
        let v20: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v21: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7, v20));
        let v22: Rc<UH2> = Rc::new(UH2::UH2_Cons(v9, v21));
        let v23: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v24: Rc<UH3> = method14(v5, v23);
        let v25: US0 = table_190(v22, v24);
        let v26: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v27: Rc<UH2> = Rc::new(UH2::UH2_Cons(v25, v26));
        let v28: Rc<UH2> = Rc::new(UH2::UH2_Cons(v19, v27));
        let v29: Rc<UH2> = Rc::new(UH2::UH2_Cons(v14, v28));
        let v30: US0 = concat_193(v29);
        markup_text_13(v30)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rx_conformance_html(v0: i32) -> Rc<str> {
    closure0()(v0)
}
pub type SpiralShape_652accfc6a8d = Mut0;
