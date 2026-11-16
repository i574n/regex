#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
enum US0 {
    US0_EndsInZero,
    US0_ZeroRuns,
    US0_FourthFromEnd,
    US0_CountedLetters,
    US0_FourthFromEndLong,
    US0_EighthFromEnd,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(u32, u32, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Normal(Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Bounded(u32),
    US2_Unbounded,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nothing,
    UH0_Epsilon,
    UH0_Set(US1),
    UH0_Seq(Rc<UH0>, Rc<UH0>),
    UH0_Alt(Rc<UH0>, Rc<UH0>),
    UH0_And(Rc<UH0>, Rc<UH0>),
    UH0_Not(Rc<UH0>),
    UH0_Star(Rc<UH0>),
    UH0_Repeat(Rc<UH0>, u32, US2),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<UH0>, Rc<UH2>),
}
#[derive(Clone)]
enum US3 {
    US3_Before,
    US3_Same,
    US3_After,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(u32, Rc<UH3>),
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<UH0>, i32, Rc<UH4>),
}
#[derive(Clone)]
enum US4 {
    US4_Accepted,
    US4_Rejected,
}
#[derive(Clone)]
enum US5 {
    US5_Dead,
    US5_To(i32),
}
#[derive(Clone)]
enum UH6 {
    UH6_Nil,
    UH6_Cons(US5, Rc<UH6>),
}
#[derive(Clone)]
enum UH5 {
    UH5_Nil,
    UH5_Cons(US4, Rc<UH6>, Rc<UH5>),
}
#[derive(Clone)]
enum US6 {
    US6_Built(Rc<UH1>, US5, Rc<UH5>, Rc<UH2>),
    US6_StateCapReached(i32),
}
#[derive(Clone)]
enum UH8 {
    UH8_NothingRejects,
    UH8_SetRejects,
    UH8_SeqLeftRejects(Rc<UH8>),
    UH8_SeqRightRejects(Rc<UH8>),
    UH8_AltRejects(Rc<UH8>, Rc<UH8>),
    UH8_AndLeftRejects(Rc<UH8>),
    UH8_AndRightRejects(Rc<UH8>),
    UH8_NotOfAccepting(Rc<UH7>),
    UH8_RepeatRejects(Rc<UH8>),
}
#[derive(Clone)]
enum UH7 {
    UH7_EpsilonAccepts,
    UH7_StarAccepts,
    UH7_RepeatFromZero,
    UH7_RepeatOfNullable(Rc<UH7>),
    UH7_SeqAccepts(Rc<UH7>, Rc<UH7>),
    UH7_AltLeftAccepts(Rc<UH7>),
    UH7_AltRightAccepts(Rc<UH7>),
    UH7_AndAccepts(Rc<UH7>, Rc<UH7>),
    UH7_NotOfRejecting(Rc<UH8>),
}
#[derive(Clone)]
enum US7 {
    US7_Accepts(Rc<UH7>),
    US7_Rejects(Rc<UH8>),
}
#[derive(Clone)]
enum US8 {
    US8_Known(i32),
    US8_Unknown,
}
#[derive(Clone)]
enum US9 {
    US9_DeadBlock,
    US9_Block(i32),
}
#[derive(Clone)]
enum UH9 {
    UH9_Nil,
    UH9_Cons(US9, Rc<UH9>),
}
#[derive(Clone)]
enum UH10 {
    UH10_Nil,
    UH10_Cons(US9, Rc<UH9>, i32, Rc<UH10>),
}
#[derive(Clone)]
enum US10 {
    US10_FoundAt(i32),
    US10_Missing,
}
#[derive(Clone)]
enum UH11 {
    UH11_Nil,
    UH11_Cons(bool, Rc<UH11>),
}
#[derive(Clone)]
enum UH12 {
    UH12_Nil,
    UH12_Cons(i32, Rc<UH12>),
}
#[derive(Clone)]
enum US11 {
    US11_Some(i32),
    US11_None,
}
#[derive(Clone)]
enum UH13 {
    UH13_Nil,
    UH13_Cons(i32, i32, Rc<UH3>, Rc<UH13>),
}
#[derive(Clone)]
enum US12 {
    US12_Minimal(Rc<UH13>),
    US12_NotMinimal(i32, i32),
}
#[derive(Clone)]
enum UH14 {
    UH14_Nil,
    UH14_Cons(US5, US5, Rc<UH3>, Rc<UH14>),
}
#[derive(Clone)]
enum UH15 {
    UH15_Nil,
    UH15_Cons(US5, US5, Rc<UH15>),
}
#[derive(Clone)]
enum US13 {
    US13_Equivalent(i32),
    US13_Distinguished(Rc<UH3>),
}
#[derive(Clone)]
enum US14 {
    US14_Certified(i32),
    US14_Forged(i32, i32, Rc<UH3>),
    US14_Uncertified(i32, i32),
}
#[derive(Clone)]
enum UH16 {
    UH16_CertifiedAutomaton(Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, i32),
}
#[derive(Clone)]
enum US15 {
    US15_Some(Rc<UH16>),
    US15_None,
}
#[derive(Clone)]
enum US16 {
    US16_Checked(US14),
    US16_TooLarge(i32),
}
#[derive(Clone)]
enum US17 {
    US17_Agree(i32),
    US17_Disagree(Rc<UH3>, US4, US4, US4, US4),
}
#[derive(Clone)]
enum US18 {
    US18_States(i32),
    US18_Refused(i32),
}
fn insert_by_low_1(mut v0: u32, mut v1: u32, mut v2: Rc<UH1>) -> Rc<UH1> {
    match &*v2 {
        UH1::UH1_Cons(v3, v4, v5) => {
            let v3: u32 = *v3;
            let mut v4: u32 = *v4;
            let v5: Rc<UH1> = v5.clone();
            let v6: bool = v0 <= v3;
            if v6 {
                Rc::new(UH1::UH1_Cons(v0, v1, v2))
            } else {
                let v8: Rc<UH1> = insert_by_low_1(v0, v1, v5);
                Rc::new(UH1::UH1_Cons(v3, v4, v8))
            }
        }
        UH1::UH1_Nil => {
            let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_Cons(v0, v1, v11))
        }
    }
}
fn sorted_by_low_0(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: u32 = v2;
                let v3: u32 = v3;
                let mut v4: Rc<UH1> = v4;
                let v5: bool = v2 <= v3;
                let v7: Rc<UH1> = if v5 {
                    insert_by_low_1(v2, v3, v1)
                } else {
                    v1
                };
                (v0, v1) = (v4, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn merge_pair_3(mut v0: u32, mut v1: u32, mut v2: u32, mut v3: u32, mut v4: Rc<UH1>) -> Rc<UH1> {
    let v5: u32 = v1.wrapping_add(1u32);
    let v6: bool = v2 <= v5;
    if v6 {
        let v7: bool = v1 < v3;
        let v8: u32 = if v7 {
            v3
        } else {
            v1
        };
        Rc::new(UH1::UH1_Cons(v0, v8, v4))
    } else {
        let v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4));
        Rc::new(UH1::UH1_Cons(v0, v1, v10))
    }
}
fn merged_2(mut v0: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v1, v2, v3) => {
            let mut v1: u32 = v1;
            let mut v2: u32 = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = merged_2(v3);
            match Rc::unwrap_or_clone(v4) {
                UH1::UH1_Cons(v5, v6, v7) => {
                    let v5: u32 = v5;
                    let v6: u32 = v6;
                    let v7: Rc<UH1> = v7;
                    merge_pair_3(v1, v2, v5, v6, v7)
                }
                UH1::UH1_Nil => {
                    let v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v1, v2, v9))
                }
            }
        }
        UH1::UH1_Nil => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn bounded_repeat_5(mut v0: Rc<UH0>, mut v1: u32, mut v2: u32, mut v3: US2) -> Rc<UH0> {
    let mut v4: bool = v2 == 0u32;
    if v4 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Epsilon); } CASE.with(|case| case.clone()) }
    } else {
        let v6: bool = v2 < v1;
        if v6 {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
        } else {
            let v8: bool = v1 == 1u32;
            let v10: bool = if v8 {
                let v9: bool = v2 == 1u32;
                v9
            } else {
                false
            };
            if v10 {
                v0
            } else {
                match &*v0 {
                    UH0::UH0_Epsilon => {
                        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Epsilon); } CASE.with(|case| case.clone()) }
                    }
                    UH0::UH0_Nothing => {
                        let v11: bool = v1 == 0u32;
                        if v11 {
                            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Epsilon); } CASE.with(|case| case.clone()) }
                        } else {
                            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
                        }
                    }
                    _ => {
                        Rc::new(UH0::UH0_Repeat(v0, v1, v3))
                    }
                }
            }
        }
    }
}
fn star_6(mut v0: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Epsilon => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Epsilon); } CASE.with(|case| case.clone()) }
        }
        UH0::UH0_Nothing => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Epsilon); } CASE.with(|case| case.clone()) }
        }
        UH0::UH0_Star(_) => {
            v0
        }
        _ => {
            Rc::new(UH0::UH0_Star(v0))
        }
    }
}
fn repeat_4(mut v0: Rc<UH0>, mut v1: u32, mut v2: US2) -> Rc<UH0> {
    match &v2 {
        US2::US2_Bounded(v3) => {
            let v3: u32 = *v3;
            bounded_repeat_5(v0, v1, v3, v2)
        }
        US2::US2_Unbounded => {
            let v5: bool = v1 == 0u32;
            if v5 {
                star_6(v0)
            } else {
                match &*v0 {
                    UH0::UH0_Epsilon => {
                        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Epsilon); } CASE.with(|case| case.clone()) }
                    }
                    UH0::UH0_Nothing => {
                        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
                    }
                    _ => {
                        Rc::new(UH0::UH0_Repeat(v0, v1, v2))
                    }
                }
            }
        }
    }
}
fn alt_of_8(mut v0: Rc<UH2>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<UH0> = v2;
            let v3: Rc<UH2> = v3;
            match &*v3 {
                UH2::UH2_Nil => {
                    v2
                }
                _ => {
                    let mut v4: Rc<UH0> = alt_of_8(v3);
                    Rc::new(UH0::UH0_Alt(v2, v4))
                }
            }
        }
        UH2::UH2_Nil => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
        }
    }
}
fn after_tie_16(mut v0: US3, mut v1: US3) -> US3 {
    match &v0 {
        US3::US3_Same => {
            v1
        }
        _ => {
            v0
        }
    }
}
fn order_empty_17(mut v0: Rc<UH1>) -> US3 {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Nil => {
            US3::US3_Same
        }
        _ => {
            US3::US3_Before
        }
    }
}
fn order_range_18(mut v0: u32, mut v1: u32, mut v2: u32, mut v3: u32) -> US3 {
    let mut v4: bool = v0 < v2;
    let v10: US3 = if v4 {
        US3::US3_Before
    } else {
        let v6: bool = v0 > v2;
        if v6 {
            US3::US3_After
        } else {
            US3::US3_Same
        }
    };
    match &v10 {
        US3::US3_Same => {
            let v11: bool = v1 < v3;
            if v11 {
                US3::US3_Before
            } else {
                let v13: bool = v1 > v3;
                if v13 {
                    US3::US3_After
                } else {
                    US3::US3_Same
                }
            }
        }
        _ => {
            v10
        }
    }
}
fn order_ranges_15(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> US3 {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let v3: u32 = v3;
            let mut v4: u32 = v4;
            let v5: Rc<UH1> = v5;
            match Rc::unwrap_or_clone(v1) {
                UH1::UH1_Cons(v7, v8, v9) => {
                    let v7: u32 = v7;
                    let v8: u32 = v8;
                    let v9: Rc<UH1> = v9;
                    let v10: US3 = order_range_18(v3, v4, v7, v8);
                    let v11: US3 = order_ranges_15(v5, v9);
                    after_tie_16(v10, v11)
                }
                UH1::UH1_Nil => {
                    US3::US3_After
                }
            }
        }
        UH1::UH1_Nil => {
            order_empty_17(v1)
        }
    }
}
fn order_sets_14(mut v0: US1, mut v1: US1) -> US3 {
    let v3: Rc<UH1> = match v0 {
        US1::US1_Normal(v2) => {
            let mut v2: Rc<UH1> = v2;
            v2
        }
    };
    let v5: Rc<UH1> = match v1 {
        US1::US1_Normal(v4) => {
            let mut v4: Rc<UH1> = v4;
            v4
        }
    };
    order_ranges_15(v3, v5)
}
enum Group11 {
    M11(Rc<UH0>, Rc<UH0>),
    M12(Rc<UH0>, Rc<UH0>),
    M13(Rc<UH0>, Rc<UH0>, Rc<UH0>, Rc<UH0>),
    M19(Rc<UH0>, Rc<UH0>),
    M20(Rc<UH0>, Rc<UH0>),
    M21(Rc<UH0>, u32, US2, Rc<UH0>),
    M22(Rc<UH0>, u32, US2, Rc<UH0>, u32, US2),
}
fn group_11(mut st: Group11) -> US3 {
    loop {
        match st {
            Group11::M11(mut v0, mut v1) => {
                let v22: u32 = match &*v0 {
                    UH0::UH0_Alt(_, _) => {
                        4u32
                    }
                    UH0::UH0_And(_, _) => {
                        5u32
                    }
                    UH0::UH0_Epsilon => {
                        1u32
                    }
                    UH0::UH0_Not(_) => {
                        6u32
                    }
                    UH0::UH0_Nothing => {
                        0u32
                    }
                    UH0::UH0_Repeat(_, _, _) => {
                        8u32
                    }
                    UH0::UH0_Seq(_, _) => {
                        3u32
                    }
                    UH0::UH0_Set(_) => {
                        2u32
                    }
                    UH0::UH0_Star(_) => {
                        7u32
                    }
                };
                let v43: u32 = match &*v1 {
                    UH0::UH0_Alt(_, _) => {
                        4u32
                    }
                    UH0::UH0_And(_, _) => {
                        5u32
                    }
                    UH0::UH0_Epsilon => {
                        1u32
                    }
                    UH0::UH0_Not(_) => {
                        6u32
                    }
                    UH0::UH0_Nothing => {
                        0u32
                    }
                    UH0::UH0_Repeat(_, _, _) => {
                        8u32
                    }
                    UH0::UH0_Seq(_, _) => {
                        3u32
                    }
                    UH0::UH0_Set(_) => {
                        2u32
                    }
                    UH0::UH0_Star(_) => {
                        7u32
                    }
                };
                let v44: bool = v22 < v43;
                let v50: US3 = if v44 {
                    US3::US3_Before
                } else {
                    let v46: bool = v22 > v43;
                    if v46 {
                        US3::US3_After
                    } else {
                        US3::US3_Same
                    }
                };
                match &v50 {
                    US3::US3_Same => {
                        st = Group11::M12(v0, v1);
                        continue;
                    }
                    _ => {
                        return v50;
                    }
                }
            }
            Group11::M12(mut v0, mut v1) => {
                match &*v0 {
                    UH0::UH0_Seq(v7, v8) => {
                        let v7: Rc<UH0> = v7.clone();
                        let v8: Rc<UH0> = v8.clone();
                        match Rc::unwrap_or_clone(v1) {
                            UH0::UH0_Seq(v9, v10) => {
                                let v9: Rc<UH0> = v9;
                                let v10: Rc<UH0> = v10;
                                st = Group11::M13(v7, v8, v9, v10);
                                continue;
                            }
                            _ => {
                                return US3::US3_Same;
                            }
                        }
                    }
                    UH0::UH0_Set(v2) => {
                        let mut v2: US1 = v2.clone();
                        match Rc::unwrap_or_clone(v1) {
                            UH0::UH0_Set(v3) => {
                                let v3: US1 = v3;
                                return order_sets_14(v2, v3);
                            }
                            _ => {
                                return US3::US3_Same;
                            }
                        }
                    }
                    _ => {
                        st = Group11::M19(v0, v1);
                        continue;
                    }
                }
            }
            Group11::M13(mut v0, mut v1, mut v2, mut v3) => {
                let mut v4: US3 = order_re_11(v0, v2);
                match &v4 {
                    US3::US3_Same => {
                        st = Group11::M11(v1, v3);
                        continue;
                    }
                    _ => {
                        return v4;
                    }
                }
            }
            Group11::M19(mut v0, mut v1) => {
                match &*v0 {
                    UH0::UH0_Alt(v2, v3) => {
                        let mut v2: Rc<UH0> = v2.clone();
                        let v3: Rc<UH0> = v3.clone();
                        match Rc::unwrap_or_clone(v1) {
                            UH0::UH0_Alt(v4, v5) => {
                                let mut v4: Rc<UH0> = v4;
                                let v5: Rc<UH0> = v5;
                                st = Group11::M13(v2, v3, v4, v5);
                                continue;
                            }
                            _ => {
                                return US3::US3_Same;
                            }
                        }
                    }
                    UH0::UH0_And(v9, v10) => {
                        let v9: Rc<UH0> = v9.clone();
                        let v10: Rc<UH0> = v10.clone();
                        match Rc::unwrap_or_clone(v1) {
                            UH0::UH0_And(v11, v12) => {
                                let v11: Rc<UH0> = v11;
                                let v12: Rc<UH0> = v12;
                                st = Group11::M13(v9, v10, v11, v12);
                                continue;
                            }
                            _ => {
                                return US3::US3_Same;
                            }
                        }
                    }
                    _ => {
                        st = Group11::M20(v0, v1);
                        continue;
                    }
                }
            }
            Group11::M20(mut v0, mut v1) => {
                match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Not(v2) => {
                        let mut v2: Rc<UH0> = v2;
                        match Rc::unwrap_or_clone(v1) {
                            UH0::UH0_Not(v3) => {
                                let v3: Rc<UH0> = v3;
                                st = Group11::M11(v2, v3);
                                continue;
                            }
                            _ => {
                                return US3::US3_Same;
                            }
                        }
                    }
                    UH0::UH0_Repeat(v12, v13, v14) => {
                        let v12: Rc<UH0> = v12;
                        let v13: u32 = v13;
                        let v14: US2 = v14;
                        st = Group11::M21(v12, v13, v14, v1);
                        continue;
                    }
                    UH0::UH0_Star(v7) => {
                        let v7: Rc<UH0> = v7;
                        match Rc::unwrap_or_clone(v1) {
                            UH0::UH0_Star(v8) => {
                                let v8: Rc<UH0> = v8;
                                st = Group11::M11(v7, v8);
                                continue;
                            }
                            _ => {
                                return US3::US3_Same;
                            }
                        }
                    }
                    _ => {
                        return US3::US3_Same;
                    }
                }
            }
            Group11::M21(mut v0, mut v1, mut v2, mut v3) => {
                match Rc::unwrap_or_clone(v3) {
                    UH0::UH0_Repeat(v4, v5, v6) => {
                        let mut v4: Rc<UH0> = v4;
                        let v5: u32 = v5;
                        let v6: US2 = v6;
                        st = Group11::M22(v0, v1, v2, v4, v5, v6);
                        continue;
                    }
                    _ => {
                        return US3::US3_Same;
                    }
                }
            }
            Group11::M22(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5) => {
                let v6: bool = v1 < v4;
                let v12: US3 = if v6 {
                    US3::US3_Before
                } else {
                    let v8: bool = v1 > v4;
                    if v8 {
                        US3::US3_After
                    } else {
                        US3::US3_Same
                    }
                };
                match &v12 {
                    US3::US3_Same => {
                        let v31: US3 = match v2 {
                            US2::US2_Bounded(v13) => {
                                let v13: u32 = v13;
                                match v5 {
                                    US2::US2_Bounded(v14) => {
                                        let v14: u32 = v14;
                                        let v15: bool = v13 < v14;
                                        if v15 {
                                            US3::US3_Before
                                        } else {
                                            let v17: bool = v13 > v14;
                                            if v17 {
                                                US3::US3_After
                                            } else {
                                                US3::US3_Same
                                            }
                                        }
                                    }
                                    US2::US2_Unbounded => {
                                        US3::US3_Before
                                    }
                                }
                            }
                            US2::US2_Unbounded => {
                                match v5 {
                                    US2::US2_Bounded(_) => {
                                        US3::US3_After
                                    }
                                    US2::US2_Unbounded => {
                                        US3::US3_Same
                                    }
                                }
                            }
                        };
                        match &v31 {
                            US3::US3_Same => {
                                st = Group11::M11(v0, v3);
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
fn order_repeat_22(mut v0: Rc<UH0>, mut v1: u32, mut v2: US2, mut v3: Rc<UH0>, mut v4: u32, mut v5: US2) -> US3 {
    group_11(Group11::M22(v0, v1, v2, v3, v4, v5))
}
fn order_repeat_with_21(mut v0: Rc<UH0>, mut v1: u32, mut v2: US2, mut v3: Rc<UH0>) -> US3 {
    group_11(Group11::M21(v0, v1, v2, v3))
}
fn order_unary_20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> US3 {
    group_11(Group11::M20(v0, v1))
}
fn order_pair_19(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> US3 {
    group_11(Group11::M19(v0, v1))
}
fn order_two_13(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> US3 {
    group_11(Group11::M13(v0, v1, v2, v3))
}
fn order_same_12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> US3 {
    group_11(Group11::M12(v0, v1))
}
fn order_re_11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> US3 {
    group_11(Group11::M11(v0, v1))
}
fn insert_leaf_10(mut v0: Rc<UH0>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match &*v1 {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH2> = v3.clone();
            let mut v4: US3 = order_re_11(v0.clone(), v2.clone());
            match v4 {
                US3::US3_After => {
                    let v6: Rc<UH2> = insert_leaf_10(v0, v3);
                    Rc::new(UH2::UH2_Cons(v2, v6))
                }
                US3::US3_Before => {
                    Rc::new(UH2::UH2_Cons(v0, v1))
                }
                US3::US3_Same => {
                    v1
                }
            }
        }
        UH2::UH2_Nil => {
            let v11: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v0, v11))
        }
    }
}
fn alt_leaves_9(mut v0: Rc<UH0>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH0::UH0_Alt(v2, v3) => {
                let mut v2: Rc<UH0> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH2> = alt_leaves_9(v3, v1);
                (v0, v1) = (v2, v4);
                continue;
            }
            UH0::UH0_Nothing => {
                return v1;
            }
            _ => {
                return insert_leaf_10(v0, v1);
            }
        }
    }
}
fn alt_7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    let mut v4: bool = match &*v0 {
        UH0::UH0_Not(v2) => {
            let mut v2: Rc<UH0> = v2.clone();
            match Rc::unwrap_or_clone(v2) {
                UH0::UH0_Nothing => {
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
            UH0::UH0_Not(v5) => {
                let v5: Rc<UH0> = v5.clone();
                match Rc::unwrap_or_clone(v5) {
                    UH0::UH0_Nothing => {
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
        let v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Not(v9))
    } else {
        let v11: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v12: Rc<UH2> = alt_leaves_9(v1, v11);
        let v13: Rc<UH2> = alt_leaves_9(v0, v12);
        alt_of_8(v13)
    }
}
fn same_re_25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> bool {
    let mut v2: US3 = order_re_11(v0, v1);
    match v2 {
        US3::US3_Same => {
            true
        }
        _ => {
            false
        }
    }
}
fn starred_twice_24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> bool {
    match &*v0 {
        UH0::UH0_Star(_) => {
            match &*v1 {
                UH0::UH0_Seq(v3, _) => {
                    let v3: Rc<UH0> = v3.clone();
                    same_re_25(v0, v3)
                }
                _ => {
                    same_re_25(v0, v1)
                }
            }
        }
        _ => {
            false
        }
    }
}
fn seq_23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Epsilon => {
                return v1;
            }
            UH0::UH0_Nothing => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) };
            }
            UH0::UH0_Seq(v3, v4) => {
                let v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let v5: Rc<UH0> = seq_23(v4, v1);
                (v0, v1) = (v3, v5);
                continue;
            }
            _ => {
                match &*v1 {
                    UH0::UH0_Epsilon => {
                        return v0;
                    }
                    UH0::UH0_Nothing => {
                        return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) };
                    }
                    _ => {
                        let v8: bool = starred_twice_24(v0.clone(), v1.clone());
                        if v8 {
                            return v1;
                        } else {
                            return Rc::new(UH0::UH0_Seq(v0, v1));
                        }
                    }
                }
            }
        }
    }
}
fn insert_point_30(mut v0: u32, mut v1: Rc<UH3>) -> Rc<UH3> {
    match &*v1 {
        UH3::UH3_Cons(v2, v3) => {
            let mut v2: u32 = *v2;
            let v3: Rc<UH3> = v3.clone();
            let mut v4: bool = v0 == v2;
            if v4 {
                v1
            } else {
                let v5: bool = v0 < v2;
                if v5 {
                    Rc::new(UH3::UH3_Cons(v0, v1))
                } else {
                    let v7: Rc<UH3> = insert_point_30(v0, v3);
                    Rc::new(UH3::UH3_Cons(v2, v7))
                }
            }
        }
        UH3::UH3_Nil => {
            let v11: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH3::UH3_Cons(v0, v11))
        }
    }
}
fn with_range_29(mut v0: u32, mut v1: u32, mut v2: Rc<UH3>) -> Rc<UH3> {
    let v3: Rc<UH3> = insert_point_30(v0, v2);
    let mut v4: bool = v1 < 1114111u32;
    if v4 {
        let v5: u32 = v1.wrapping_add(1u32);
        insert_point_30(v5, v3)
    } else {
        v3
    }
}
fn range_points_28(mut v0: Rc<UH1>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: u32 = v2;
                let v3: u32 = v3;
                let mut v4: Rc<UH1> = v4;
                let v5: Rc<UH3> = with_range_29(v2, v3, v1);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn parts_of_31(mut v0: &Rc<UH0>) -> Rc<UH2> {
    match &**v0 {
        UH0::UH0_Alt(v6, v7) => {
            let v6: Rc<UH0> = v6.clone();
            let v7: Rc<UH0> = v7.clone();
            let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v9: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7, v8));
            Rc::new(UH2::UH2_Cons(v6, v9))
        }
        UH0::UH0_And(v11, v12) => {
            let v11: Rc<UH0> = v11.clone();
            let v12: Rc<UH0> = v12.clone();
            let v13: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v14: Rc<UH2> = Rc::new(UH2::UH2_Cons(v12, v13));
            Rc::new(UH2::UH2_Cons(v11, v14))
        }
        UH0::UH0_Not(v16) => {
            let v16: Rc<UH0> = v16.clone();
            let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v16, v17))
        }
        UH0::UH0_Repeat(v22, _, _) => {
            let v22: Rc<UH0> = v22.clone();
            let v25: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v22, v25))
        }
        UH0::UH0_Seq(v1, v2) => {
            let mut v1: Rc<UH0> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let mut v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3));
            Rc::new(UH2::UH2_Cons(v1, v4))
        }
        UH0::UH0_Star(v19) => {
            let v19: Rc<UH0> = v19.clone();
            let v20: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v19, v20))
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method32(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<UH0> = v2;
            let v3: Rc<UH2> = v3;
            let mut v4: Rc<UH2> = method32(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn points_in_27(mut v0: Rc<UH2>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<UH0> = v2;
                let v3: Rc<UH2> = v3;
                match &*v2 {
                    UH0::UH0_Set(v4) => {
                        let mut v4: US1 = v4.clone();
                        let v6: Rc<UH1> = match v4 {
                            US1::US1_Normal(v5) => {
                                let v5: Rc<UH1> = v5;
                                v5
                            }
                        };
                        let v7: Rc<UH3> = range_points_28(v6, v1);
                        (v0, v1) = (v3, v7);
                        continue;
                    }
                    _ => {
                        let v9: Rc<UH2> = parts_of_31(&v2);
                        let v10: Rc<UH2> = method32(v9, v3);
                        v0 = v10;
                        continue;
                    }
                }
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn points_of_26(mut v0: Rc<UH0>, mut v1: Rc<UH3>) -> Rc<UH3> {
    let mut v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH2> = Rc::new(UH2::UH2_Cons(v0, v2));
    points_in_27(v3, v1)
}
fn classes_from_33(mut v0: Rc<UH3>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v1, v2) => {
            let mut v1: u32 = v1;
            let mut v2: Rc<UH3> = v2;
            match &*v2 {
                UH3::UH3_Cons(v3, _) => {
                    let v3: u32 = *v3;
                    let v5: u32 = v3.wrapping_sub(1u32);
                    let v6: Rc<UH1> = classes_from_33(v2);
                    Rc::new(UH1::UH1_Cons(v1, v5, v6))
                }
                UH3::UH3_Nil => {
                    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v1, 1114111u32, v8))
                }
            }
        }
        UH3::UH3_Nil => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method34(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(_, _, v4) => {
                let mut v4: Rc<UH1> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn start_target_38(mut v0: &Rc<UH0>) -> US5 {
    let mut v1: bool = match &**v0 {
        UH0::UH0_Nothing => {
            true
        }
        _ => {
            false
        }
    };
    if v1 {
        US5::US5_Dead
    } else {
        US5::US5_To(0i32)
    }
}
fn method39(mut v0: Rc<UH5>, mut v1: Rc<UH5>) -> Rc<UH5> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH5::UH5_Cons(v2, v3, v4) => {
                let mut v2: US4 = v2;
                let v3: Rc<UH6> = v3;
                let mut v4: Rc<UH5> = v4;
                let v5: Rc<UH5> = Rc::new(UH5::UH5_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH5::UH5_Nil => {
                return v1;
            }
        }
    }
}
fn re_at_41(mut v0: i32, mut v1: Rc<UH4>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH4::UH4_Cons(v2, v3, v4) => {
                let mut v2: Rc<UH0> = v2;
                let v3: i32 = v3;
                let mut v4: Rc<UH4> = v4;
                let v5: bool = v3 == v0;
                if v5 {
                    return v2;
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH4::UH4_Nil => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn regexes_in_order_40(mut v0: i32, mut v1: Rc<UH4>, mut v2: i32, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        let mut v4: bool = v0 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.wrapping_sub(1i32);
            let v6: Rc<UH0> = re_at_41(v0, v1.clone());
            let v7: Rc<UH2> = Rc::new(UH2::UH2_Cons(v6, v3));
            (v0, v3) = (v5, v7);
            continue;
        }
    }
}
fn built_dfa_37(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<UH4>, mut v3: i32, mut v4: Rc<UH5>) -> US6 {
    let v5: US5 = start_target_38(&v0);
    let v6: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH5> = method39(v4, v6);
    let v8: i32 = v3.wrapping_sub(1i32);
    let v9: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH2> = regexes_in_order_40(v8, v2, v3, v9);
    US6::US6_Built(v1, v5, v7, v10)
}
fn lows_of_45(mut v0: Rc<UH1>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v1, _, v3) => {
            let mut v1: u32 = v1;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH3> = lows_of_45(v3);
            Rc::new(UH3::UH3_Cons(v1, v4))
        }
        UH1::UH1_Nil => {
            { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn seq_right_52(mut v0: Rc<UH7>, mut v1: US7) -> US7 {
    match v1 {
        US7::US7_Accepts(v2) => {
            let mut v2: Rc<UH7> = v2;
            let v3: Rc<UH7> = Rc::new(UH7::UH7_SeqAccepts(v0, v2));
            US7::US7_Accepts(v3)
        }
        US7::US7_Rejects(v5) => {
            let v5: Rc<UH8> = v5;
            let v6: Rc<UH8> = Rc::new(UH8::UH8_SeqRightRejects(v5));
            US7::US7_Rejects(v6)
        }
    }
}
fn seq_nullability_51(mut v0: US7, mut v1: Rc<UH0>) -> US7 {
    match v0 {
        US7::US7_Accepts(v5) => {
            let v5: Rc<UH7> = v5;
            let v6: US7 = nullability_of_50(v1);
            seq_right_52(v5, v6)
        }
        US7::US7_Rejects(v2) => {
            let mut v2: Rc<UH8> = v2;
            let v3: Rc<UH8> = Rc::new(UH8::UH8_SeqLeftRejects(v2));
            US7::US7_Rejects(v3)
        }
    }
}
fn alt_right_55(mut v0: Rc<UH8>, mut v1: US7) -> US7 {
    match v1 {
        US7::US7_Accepts(v2) => {
            let mut v2: Rc<UH7> = v2;
            let v3: Rc<UH7> = Rc::new(UH7::UH7_AltRightAccepts(v2));
            US7::US7_Accepts(v3)
        }
        US7::US7_Rejects(v5) => {
            let v5: Rc<UH8> = v5;
            let v6: Rc<UH8> = Rc::new(UH8::UH8_AltRejects(v0, v5));
            US7::US7_Rejects(v6)
        }
    }
}
fn alt_nullability_54(mut v0: US7, mut v1: Rc<UH0>) -> US7 {
    match v0 {
        US7::US7_Accepts(v2) => {
            let mut v2: Rc<UH7> = v2;
            let v3: Rc<UH7> = Rc::new(UH7::UH7_AltLeftAccepts(v2));
            US7::US7_Accepts(v3)
        }
        US7::US7_Rejects(v5) => {
            let v5: Rc<UH8> = v5;
            let v6: US7 = nullability_of_50(v1);
            alt_right_55(v5, v6)
        }
    }
}
fn and_right_57(mut v0: Rc<UH7>, mut v1: US7) -> US7 {
    match v1 {
        US7::US7_Accepts(v2) => {
            let mut v2: Rc<UH7> = v2;
            let v3: Rc<UH7> = Rc::new(UH7::UH7_AndAccepts(v0, v2));
            US7::US7_Accepts(v3)
        }
        US7::US7_Rejects(v5) => {
            let v5: Rc<UH8> = v5;
            let v6: Rc<UH8> = Rc::new(UH8::UH8_AndRightRejects(v5));
            US7::US7_Rejects(v6)
        }
    }
}
fn and_nullability_56(mut v0: US7, mut v1: Rc<UH0>) -> US7 {
    match v0 {
        US7::US7_Accepts(v5) => {
            let v5: Rc<UH7> = v5;
            let v6: US7 = nullability_of_50(v1);
            and_right_57(v5, v6)
        }
        US7::US7_Rejects(v2) => {
            let mut v2: Rc<UH8> = v2;
            let v3: Rc<UH8> = Rc::new(UH8::UH8_AndLeftRejects(v2));
            US7::US7_Rejects(v3)
        }
    }
}
fn repeat_null_59(mut v0: Rc<UH0>, mut v1: u32) -> US7 {
    let mut v2: bool = v1 == 0u32;
    if v2 {
        let v3: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_RepeatFromZero); } CASE.with(|case| case.clone()) };
        US7::US7_Accepts(v3)
    } else {
        let v5: US7 = nullability_of_50(v0);
        match v5 {
            US7::US7_Accepts(v6) => {
                let v6: Rc<UH7> = v6;
                let v7: Rc<UH7> = Rc::new(UH7::UH7_RepeatOfNullable(v6));
                US7::US7_Accepts(v7)
            }
            US7::US7_Rejects(v9) => {
                let v9: Rc<UH8> = v9;
                let v10: Rc<UH8> = Rc::new(UH8::UH8_RepeatRejects(v9));
                US7::US7_Rejects(v10)
            }
        }
    }
}
fn nullability_loop_58(mut v0: Rc<UH0>) -> US7 {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Not(v1) => {
            let mut v1: Rc<UH0> = v1;
            let mut v2: US7 = nullability_of_50(v1);
            match v2 {
                US7::US7_Accepts(v3) => {
                    let v3: Rc<UH7> = v3;
                    let mut v4: Rc<UH8> = Rc::new(UH8::UH8_NotOfAccepting(v3));
                    US7::US7_Rejects(v4)
                }
                US7::US7_Rejects(v6) => {
                    let v6: Rc<UH8> = v6;
                    let v7: Rc<UH7> = Rc::new(UH7::UH7_NotOfRejecting(v6));
                    US7::US7_Accepts(v7)
                }
            }
        }
        UH0::UH0_Repeat(v14, v15, _) => {
            let v14: Rc<UH0> = v14;
            let v15: u32 = v15;
            repeat_null_59(v14, v15)
        }
        UH0::UH0_Star(_) => {
            let v12: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_StarAccepts); } CASE.with(|case| case.clone()) };
            US7::US7_Accepts(v12)
        }
        _ => {
            let v18: Rc<UH8> = { thread_local!{ static CASE: Rc<UH8> = Rc::new(UH8::UH8_NothingRejects); } CASE.with(|case| case.clone()) };
            US7::US7_Rejects(v18)
        }
    }
}
fn nullability_rest_53(mut v0: Rc<UH0>) -> US7 {
    match &*v0 {
        UH0::UH0_Alt(v1, v2) => {
            let mut v1: Rc<UH0> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            let v3: US7 = nullability_of_50(v1);
            alt_nullability_54(v3, v2)
        }
        UH0::UH0_And(v5, v6) => {
            let v5: Rc<UH0> = v5.clone();
            let v6: Rc<UH0> = v6.clone();
            let v7: US7 = nullability_of_50(v5);
            and_nullability_56(v7, v6)
        }
        _ => {
            nullability_loop_58(v0)
        }
    }
}
fn nullability_of_50(mut v0: Rc<UH0>) -> US7 {
    match &*v0 {
        UH0::UH0_Epsilon => {
            let v3: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_EpsilonAccepts); } CASE.with(|case| case.clone()) };
            US7::US7_Accepts(v3)
        }
        UH0::UH0_Nothing => {
            let mut v1: Rc<UH8> = { thread_local!{ static CASE: Rc<UH8> = Rc::new(UH8::UH8_NothingRejects); } CASE.with(|case| case.clone()) };
            US7::US7_Rejects(v1)
        }
        UH0::UH0_Seq(v8, v9) => {
            let v8: Rc<UH0> = v8.clone();
            let v9: Rc<UH0> = v9.clone();
            let v10: US7 = nullability_of_50(v8);
            seq_nullability_51(v10, v9)
        }
        UH0::UH0_Set(_) => {
            let v6: Rc<UH8> = { thread_local!{ static CASE: Rc<UH8> = Rc::new(UH8::UH8_SetRejects); } CASE.with(|case| case.clone()) };
            US7::US7_Rejects(v6)
        }
        _ => {
            nullability_rest_53(v0)
        }
    }
}
fn nullable_49(mut v0: Rc<UH0>) -> bool {
    let mut v1: US7 = nullability_of_50(v0);
    match v1 {
        US7::US7_Accepts(_) => {
            true
        }
        US7::US7_Rejects(_) => {
            false
        }
    }
}
fn derive_seq_48(mut v0: u32, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    let v3: Rc<UH0> = derive_47(v0, v1.clone());
    let mut v4: Rc<UH0> = seq_23(v3, v2.clone());
    let v5: bool = nullable_49(v1);
    if v5 {
        let v6: Rc<UH0> = derive_47(v0, v2);
        alt_7(v4, v6)
    } else {
        v4
    }
}
fn and_of_62(mut v0: Rc<UH2>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v3, v4) => {
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH2> = v4;
            match &*v4 {
                UH2::UH2_Nil => {
                    v3
                }
                _ => {
                    let v5: Rc<UH0> = and_of_62(v4);
                    Rc::new(UH0::UH0_And(v3, v5))
                }
            }
        }
        UH2::UH2_Nil => {
            let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Not(v1))
        }
    }
}
fn and_leaves_63(mut v0: Rc<UH0>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH0::UH0_And(v2, v3) => {
                let mut v2: Rc<UH0> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH2> = and_leaves_63(v3, v1);
                (v0, v1) = (v2, v4);
                continue;
            }
            _ => {
                let v8: bool = match &*v0 {
                    UH0::UH0_Not(v6) => {
                        let v6: Rc<UH0> = v6.clone();
                        match Rc::unwrap_or_clone(v6) {
                            UH0::UH0_Nothing => {
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
                    return insert_leaf_10(v0, v1);
                }
            }
        }
    }
}
fn both_61(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    let mut v2: bool = match &*v0 {
        UH0::UH0_Nothing => {
            true
        }
        _ => {
            false
        }
    };
    let mut v4: bool = if v2 {
        true
    } else {
        match &*v1 {
            UH0::UH0_Nothing => {
                true
            }
            _ => {
                false
            }
        }
    };
    if v4 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
    } else {
        let v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v7: Rc<UH2> = and_leaves_63(v1, v6);
        let v8: Rc<UH2> = and_leaves_63(v0, v7);
        and_of_62(v8)
    }
}
fn negate_65(mut v0: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Not(v1) => {
            let mut v1: Rc<UH0> = v1.clone();
            v1
        }
        _ => {
            Rc::new(UH0::UH0_Not(v0))
        }
    }
}
fn derive_unary_64(mut v0: u32, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Not(v2) => {
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH0> = derive_47(v0, v2);
            negate_65(v3)
        }
        UH0::UH0_Repeat(v8, v9, v10) => {
            let v8: Rc<UH0> = v8.clone();
            let v9: u32 = *v9;
            let v10: US2 = v10.clone();
            let v11: Rc<UH0> = derive_47(v0, v8.clone());
            let v12: bool = v9 == 0u32;
            let v14: u32 = if v12 {
                0u32
            } else {
                let v13: u32 = v9.wrapping_sub(1u32);
                v13
            };
            let v22: US2 = match v10 {
                US2::US2_Bounded(v15) => {
                    let v15: u32 = v15;
                    let v16: bool = v15 == 0u32;
                    let v18: u32 = if v16 {
                        0u32
                    } else {
                        let v17: u32 = v15.wrapping_sub(1u32);
                        v17
                    };
                    US2::US2_Bounded(v18)
                }
                US2::US2_Unbounded => {
                    US2::US2_Unbounded
                }
            };
            let v23: Rc<UH0> = repeat_4(v8, v14, v22);
            seq_23(v11, v23)
        }
        UH0::UH0_Star(v5) => {
            let v5: Rc<UH0> = v5.clone();
            let v6: Rc<UH0> = derive_47(v0, v5);
            seq_23(v6, v1)
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
        }
    }
}
fn derive_rest_60(mut v0: u32, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Alt(v2, v3) => {
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = derive_47(v0, v2);
            let v5: Rc<UH0> = derive_47(v0, v3);
            alt_7(v4, v5)
        }
        UH0::UH0_And(v7, v8) => {
            let v7: Rc<UH0> = v7.clone();
            let v8: Rc<UH0> = v8.clone();
            let v9: Rc<UH0> = derive_47(v0, v7);
            let v10: Rc<UH0> = derive_47(v0, v8);
            both_61(v9, v10)
        }
        _ => {
            derive_unary_64(v0, v1)
        }
    }
}
fn contains_in_67(mut v0: u32, mut v1: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: u32 = v2;
                let v3: u32 = v3;
                let mut v4: Rc<UH1> = v4;
                let v5: bool = v0 < v2;
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
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn contains_66(mut v0: u32, mut v1: US1) -> bool {
    let v3: Rc<UH1> = match v1 {
        US1::US1_Normal(v2) => {
            let mut v2: Rc<UH1> = v2;
            v2
        }
    };
    contains_in_67(v0, v3)
}
fn derive_47(mut v0: u32, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Epsilon => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
        }
        UH0::UH0_Nothing => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
        }
        UH0::UH0_Seq(v9, v10) => {
            let v9: Rc<UH0> = v9.clone();
            let v10: Rc<UH0> = v10.clone();
            derive_seq_48(v0, v9, v10)
        }
        UH0::UH0_Set(v4) => {
            let mut v4: US1 = v4.clone();
            let v5: bool = contains_66(v0, v4);
            if v5 {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Epsilon); } CASE.with(|case| case.clone()) }
            } else {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) }
            }
        }
        _ => {
            derive_rest_60(v0, v1)
        }
    }
}
fn find_state_69(mut v0: Rc<UH0>, mut v1: Rc<UH4>) -> US8 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH4::UH4_Cons(v2, v3, v4) => {
                let mut v2: Rc<UH0> = v2;
                let v3: i32 = v3;
                let mut v4: Rc<UH4> = v4;
                let v5: bool = same_re_25(v0.clone(), v2);
                if v5 {
                    return US8::US8_Known(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH4::UH4_Nil => {
                return US8::US8_Unknown;
            }
        }
    }
}
fn with_state_70(mut v0: Rc<UH0>, mut v1: Rc<UH4>, mut v2: i32) -> (Rc<UH4>, i32) {
    let v3: i32 = v2.wrapping_add(1i32);
    let mut v4: Rc<UH4> = Rc::new(UH4::UH4_Cons(v0, v2, v1));
    (v4, v3)
}
fn target_of_68(mut v0: Rc<UH0>, mut v1: Rc<UH4>, mut v2: i32) -> (US5, Rc<UH4>, i32) {
    let v3: bool = match &*v0 {
        UH0::UH0_Nothing => {
            true
        }
        _ => {
            false
        }
    };
    if v3 {
        let mut v4: US5 = US5::US5_Dead;
        (v4, v1, v2)
    } else {
        let v5: US8 = find_state_69(v0.clone(), v1.clone());
        match v5 {
            US8::US8_Known(v6) => {
                let v6: i32 = v6;
                let v7: US5 = US5::US5_To(v6);
                (v7, v1, v2)
            }
            US8::US8_Unknown => {
                let (mut v8, mut v9): (Rc<UH4>, i32) = with_state_70(v0, v1, v2);
                let v10: US5 = US5::US5_To(v2);
                (v10, v8, v9)
            }
        }
    }
}
fn method71(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> Rc<UH6> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v2, v3) => {
                let mut v2: US5 = v2;
                let v3: Rc<UH6> = v3;
                let mut v4: Rc<UH6> = Rc::new(UH6::UH6_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH6::UH6_Nil => {
                return v1;
            }
        }
    }
}
fn row_targets_46(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: Rc<UH4>, mut v3: i32, mut v4: Rc<UH6>) -> (Rc<UH6>, Rc<UH4>, i32) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v5, v6) => {
                let v5: u32 = v5;
                let v6: Rc<UH3> = v6;
                let v7: Rc<UH0> = derive_47(v5, v0.clone());
                let (mut v8, mut v9, mut v10): (US5, Rc<UH4>, i32) = target_of_68(v7, v2, v3);
                let v11: Rc<UH6> = Rc::new(UH6::UH6_Cons(v8, v4));
                (v1, v2, v3, v4) = (v6, v9, v10, v11);
                continue;
            }
            UH3::UH3_Nil => {
                let v15: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                let v16: Rc<UH6> = method71(v4, v15);
                return (v16, v2, v3);
            }
        }
    }
}
fn next_row_44(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<UH4>, mut v3: i32) -> (US4, Rc<UH6>, Rc<UH4>, i32) {
    let mut v4: Rc<UH3> = lows_of_45(v1);
    let v5: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    let (mut v6, mut v7, mut v8): (Rc<UH6>, Rc<UH4>, i32) = row_targets_46(v0.clone(), v4, v2, v3, v5);
    let v9: bool = nullable_49(v0);
    let v12: US4 = if v9 {
        US4::US4_Accepted
    } else {
        US4::US4_Rejected
    };
    (v12, v6, v7, v8)
}
enum Group36 {
    M36(i32, i32, Rc<UH0>, Rc<UH1>, Rc<UH4>, i32, Rc<UH5>),
    M42(i32, i32, Rc<UH0>, Rc<UH1>, Rc<UH4>, i32, Rc<UH5>),
    M43(i32, i32, Rc<UH0>, Rc<UH1>, Rc<UH4>, i32, Rc<UH5>),
}
fn group_36(mut st: Group36) -> US6 {
    loop {
        match st {
            Group36::M36(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6) => {
                let v7: bool = v0 >= v5;
                if v7 {
                    return built_dfa_37(v2, v3, v4, v5, v6);
                } else {
                    st = Group36::M42(v0, v1, v2, v3, v4, v5, v6);
                    continue;
                }
            }
            Group36::M42(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6) => {
                let v7: bool = v5 > v1;
                if v7 {
                    return US6::US6_StateCapReached(v5);
                } else {
                    st = Group36::M43(v0, v1, v2, v3, v4, v5, v6);
                    continue;
                }
            }
            Group36::M43(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6) => {
                let v7: Rc<UH0> = re_at_41(v0, v4.clone());
                let (mut v8, mut v9, mut v10, mut v11): (US4, Rc<UH6>, Rc<UH4>, i32) = next_row_44(v7, v3.clone(), v4, v5);
                let v12: i32 = v0.wrapping_add(1i32);
                let v13: Rc<UH5> = Rc::new(UH5::UH5_Cons(v8, v9, v6));
                st = Group36::M36(v12, v1, v2, v3, v10, v11, v13);
                continue;
            }
        }
    }
}
fn build_step_43(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<UH1>, mut v4: Rc<UH4>, mut v5: i32, mut v6: Rc<UH5>) -> US6 {
    group_36(Group36::M43(v0, v1, v2, v3, v4, v5, v6))
}
fn build_next_42(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<UH1>, mut v4: Rc<UH4>, mut v5: i32, mut v6: Rc<UH5>) -> US6 {
    group_36(Group36::M42(v0, v1, v2, v3, v4, v5, v6))
}
fn build_rows_36(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<UH1>, mut v4: Rc<UH4>, mut v5: i32, mut v6: Rc<UH5>) -> US6 {
    group_36(Group36::M36(v0, v1, v2, v3, v4, v5, v6))
}
fn build_within_35(mut v0: i32, mut v1: Rc<UH0>) -> US6 {
    let mut v2: i32 = 0i32;
    let v3: u32 = 0u32;
    let mut v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH3> = Rc::new(UH3::UH3_Cons(v3, v4));
    let v6: Rc<UH3> = points_of_26(v1.clone(), v5);
    let v7: Rc<UH1> = classes_from_33(v6);
    let v8: bool = match &*v1 {
        UH0::UH0_Nothing => {
            true
        }
        _ => {
            false
        }
    };
    let (mut v12, mut v13): (Rc<UH4>, i32) = if v8 {
        let v9: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
        (v9, 0i32)
    } else {
        let v10: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
        let v11: Rc<UH4> = Rc::new(UH4::UH4_Cons(v1.clone(), 0i32, v10));
        (v11, 1i32)
    };
    let v14: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    build_rows_36(v2, v0, v1, v7, v12, v13, v14)
}
fn new_block_78(mut v0: US9, mut v1: Rc<UH9>, mut v2: Rc<UH9>, mut v3: Rc<UH10>, mut v4: i32) -> (Rc<UH9>, Rc<UH10>, i32) {
    let v5: i32 = v4.wrapping_add(1i32);
    let v6: US9 = US9::US9_Block(v4);
    let v7: Rc<UH9> = Rc::new(UH9::UH9_Cons(v6, v2));
    let v8: Rc<UH10> = Rc::new(UH10::UH10_Cons(v0, v1, v4, v3));
    (v7, v8, v5)
}
fn method80(mut v0: Rc<UH9>, mut v1: Rc<UH9>) -> bool {
    loop {
        match (Rc::unwrap_or_clone(v1), Rc::unwrap_or_clone(v0)) {
            (UH9::UH9_Cons(v2, v3), UH9::UH9_Cons(v4, v5)) => {
                let mut v2: US9 = v2;
                let v3: Rc<UH9> = v3;
                let mut v4: US9 = v4;
                let v5: Rc<UH9> = v5;
                let v9: bool = match (v2, v4) {
                    (US9::US9_Block(v6), US9::US9_Block(v7)) => {
                        let v6: i32 = v6;
                        let v7: i32 = v7;
                        let v8: bool = v6 == v7;
                        v8
                    }
                    (US9::US9_DeadBlock, US9::US9_DeadBlock) => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v9 {
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return false;
                }
            }
            (UH9::UH9_Nil, UH9::UH9_Nil) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
fn signature_id_79(mut v0: US9, mut v1: Rc<UH9>, mut v2: Rc<UH10>) -> US10 {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH10::UH10_Cons(v3, v4, v5, v6) => {
                let v3: US9 = v3;
                let mut v4: Rc<UH9> = v4;
                let v5: i32 = v5;
                let v6: Rc<UH10> = v6;
                let v10: bool = match (&v3, &v0) {
                    (US9::US9_Block(v7), US9::US9_Block(v8)) => {
                        let v7: i32 = *v7;
                        let v8: i32 = *v8;
                        let v9: bool = v7 == v8;
                        v9
                    }
                    (US9::US9_DeadBlock, US9::US9_DeadBlock) => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v12: bool = if v10 {
                    method80(v1.clone(), v4)
                } else {
                    false
                };
                if v12 {
                    return US10::US10_FoundAt(v5);
                } else {
                    v2 = v6;
                    continue;
                }
            }
            UH10::UH10_Nil => {
                return US10::US10_Missing;
            }
        }
    }
}
fn assign_77(mut v0: US9, mut v1: Rc<UH9>, mut v2: Rc<UH9>, mut v3: Rc<UH10>, mut v4: i32) -> (Rc<UH9>, Rc<UH10>, i32) {
    let v5: US10 = signature_id_79(v0.clone(), v1.clone(), v3.clone());
    let (mut v15, mut v16, mut v17): (Rc<UH9>, Rc<UH10>, i32) = match &v5 {
        US10::US10_FoundAt(v6) => {
            let v6: i32 = *v6;
            let v7: US9 = US9::US9_Block(v6);
            let v8: Rc<UH9> = Rc::new(UH9::UH9_Cons(v7, v2));
            (v8, v3, v4)
        }
        _ => {
            let (mut v12, mut v13, mut v14): (Rc<UH9>, Rc<UH10>, i32) = match v5 {
                US10::US10_Missing => {
                    new_block_78(v0, v1, v2, v3, v4)
                }
                _ => unreachable!(),
            };
            (v12, v13, v14)
        }
    };
    (v15, v16, v17)
}
fn block_at_81(mut v0: i32, mut v1: Rc<UH9>) -> US9 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH9::UH9_Cons(v2, v3) => {
                let mut v2: US9 = v2;
                let v3: Rc<UH9> = v3;
                let mut v4: bool = v0 == 0i32;
                if v4 {
                    return v2;
                } else {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH9::UH9_Nil => {
                return US9::US9_DeadBlock;
            }
        }
    }
}
fn blocks_of_targets_82(mut v0: Rc<UH6>, mut v1: Rc<UH9>) -> Rc<UH9> {
    match Rc::unwrap_or_clone(v0) {
        UH6::UH6_Cons(v2, v3) => {
            let mut v2: US5 = v2;
            let v3: Rc<UH6> = v3;
            let v8: US9 = match v2 {
                US5::US5_Dead => {
                    US9::US9_DeadBlock
                }
                US5::US5_To(v5) => {
                    let v5: i32 = v5;
                    block_at_81(v5, v1.clone())
                }
            };
            let v9: Rc<UH9> = blocks_of_targets_82(v3, v1);
            Rc::new(UH9::UH9_Cons(v8, v9))
        }
        UH6::UH6_Nil => {
            { thread_local!{ static CASE: Rc<UH9> = Rc::new(UH9::UH9_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn refined_at_76(mut v0: i32, mut v1: US4, mut v2: Rc<UH6>, mut v3: Rc<UH9>, mut v4: Rc<UH9>, mut v5: Rc<UH10>, mut v6: i32) -> (Rc<UH9>, Rc<UH10>, i32) {
    let v7: US9 = block_at_81(v0, v3.clone());
    let (mut v14, mut v15, mut v16): (Rc<UH9>, Rc<UH10>, i32) = match &v7 {
        US9::US9_DeadBlock => {
            let v8: US9 = US9::US9_DeadBlock;
            let v9: Rc<UH9> = Rc::new(UH9::UH9_Cons(v8, v4));
            (v9, v5, v6)
        }
        _ => {
            let v10: Rc<UH9> = blocks_of_targets_82(v2, v3);
            let (mut v11, mut v12, mut v13): (Rc<UH9>, Rc<UH10>, i32) = assign_77(v7, v10, v4, v5, v6);
            (v11, v12, v13)
        }
    };
    (v14, v15, v16)
}
fn method83(mut v0: Rc<UH9>, mut v1: Rc<UH9>) -> Rc<UH9> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH9::UH9_Cons(v2, v3) => {
                let mut v2: US9 = v2;
                let v3: Rc<UH9> = v3;
                let mut v4: Rc<UH9> = Rc::new(UH9::UH9_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH9::UH9_Nil => {
                return v1;
            }
        }
    }
}
fn refine_rows_75(mut v0: Rc<UH5>, mut v1: i32, mut v2: Rc<UH9>, mut v3: Rc<UH9>, mut v4: Rc<UH10>, mut v5: i32) -> (Rc<UH9>, Rc<UH10>, i32) {
    loop {
        match &*v0 {
            UH5::UH5_Cons(v6, v7, v8) => {
                let v6: US4 = v6.clone();
                let v7: Rc<UH6> = v7.clone();
                let v8: Rc<UH5> = v8.clone();
                let v9: i32 = v1.wrapping_add(1i32);
                let (mut v10, mut v11, mut v12): (Rc<UH9>, Rc<UH10>, i32) = refined_at_76(v1, v6, v7, v2.clone(), v3, v4, v5);
                (v0, v1, v3, v4, v5) = (v8, v9, v10, v11, v12);
                continue;
            }
            _ => {
                let (mut v18, mut v19, mut v20): (Rc<UH9>, Rc<UH10>, i32) = match Rc::unwrap_or_clone(v0) {
                    UH5::UH5_Nil => {
                        let v16: Rc<UH9> = { thread_local!{ static CASE: Rc<UH9> = Rc::new(UH9::UH9_Nil); } CASE.with(|case| case.clone()) };
                        let v17: Rc<UH9> = method83(v3, v16);
                        (v17, v4, v5)
                    }
                    _ => unreachable!(),
                };
                return (v18, v19, v20);
            }
        }
    }
}
fn moore_74(mut v0: Rc<UH5>, mut v1: Rc<UH9>, mut v2: i32) -> (Rc<UH9>, i32) {
    loop {
        let v3: i32 = 0i32;
        let mut v4: Rc<UH9> = { thread_local!{ static CASE: Rc<UH9> = Rc::new(UH9::UH9_Nil); } CASE.with(|case| case.clone()) };
        let v5: Rc<UH10> = { thread_local!{ static CASE: Rc<UH10> = Rc::new(UH10::UH10_Nil); } CASE.with(|case| case.clone()) };
        let v6: i32 = 0i32;
        let (mut v7, mut v8, mut v9): (Rc<UH9>, Rc<UH10>, i32) = refine_rows_75(v0.clone(), v3, v1, v4, v5, v6);
        let v10: bool = v9 == v2;
        if v10 {
            return (v7, v9);
        } else {
            (v1, v2) = (v7, v9);
            continue;
        }
    }
}
fn count_rows_85(mut v0: Rc<UH5>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH5::UH5_Cons(_, _, v4) => {
                let mut v4: Rc<UH5> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH5::UH5_Nil => {
                return v1;
            }
        }
    }
}
fn state_count_84(mut v0: &Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: &Rc<UH2>) -> i32 {
    let mut v4: i32 = 0i32;
    count_rows_85(v2, v4)
}
fn falses_86(mut v0: i32) -> Rc<UH11> {
    let mut v1: bool = v0 <= 0i32;
    if v1 {
        { thread_local!{ static CASE: Rc<UH11> = Rc::new(UH11::UH11_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v3: i32 = v0.wrapping_sub(1i32);
        let mut v4: Rc<UH11> = falses_86(v3);
        Rc::new(UH11::UH11_Cons(false, v4))
    }
}
fn bool_at_91(mut v0: i32, mut v1: Rc<UH11>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH11::UH11_Cons(v2, v3) => {
                let mut v2: bool = v2;
                let v3: Rc<UH11> = v3;
                let mut v4: bool = v0 == 0i32;
                if v4 {
                    return v2;
                } else {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH11::UH11_Nil => {
                return false;
            }
        }
    }
}
fn any_live_90(mut v0: Rc<UH6>, mut v1: Rc<UH11>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH6::UH6_Cons(v2, v3) => {
                let mut v2: US5 = v2;
                let v3: Rc<UH6> = v3;
                let v7: bool = match v2 {
                    US5::US5_Dead => {
                        false
                    }
                    US5::US5_To(v4) => {
                        let mut v4: i32 = v4;
                        bool_at_91(v4, v1.clone())
                    }
                };
                if v7 {
                    return true;
                } else {
                    v0 = v3;
                    continue;
                }
            }
            UH6::UH6_Nil => {
                return false;
            }
        }
    }
}
fn live_rows_89(mut v0: Rc<UH5>, mut v1: Rc<UH11>) -> Rc<UH11> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v2, v3, v4) => {
            let mut v2: US4 = v2;
            let v3: Rc<UH6> = v3;
            let mut v4: Rc<UH5> = v4;
            let v5: bool = match v2 {
                US4::US4_Accepted => {
                    true
                }
                _ => {
                    false
                }
            };
            let v7: bool = if v5 {
                true
            } else {
                any_live_90(v3, v1.clone())
            };
            let v8: Rc<UH11> = live_rows_89(v4, v1);
            Rc::new(UH11::UH11_Cons(v7, v8))
        }
        UH5::UH5_Nil => {
            { thread_local!{ static CASE: Rc<UH11> = Rc::new(UH11::UH11_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn live_after_88(mut v0: &Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: &Rc<UH2>, mut v4: Rc<UH11>) -> Rc<UH11> {
    live_rows_89(v2, v4)
}
fn method92(mut v0: Rc<UH11>, mut v1: Rc<UH11>) -> bool {
    loop {
        match (Rc::unwrap_or_clone(v1), Rc::unwrap_or_clone(v0)) {
            (UH11::UH11_Cons(v2, v3), UH11::UH11_Cons(v4, v5)) => {
                let mut v2: bool = v2;
                let v3: Rc<UH11> = v3;
                let mut v4: bool = v4;
                let v5: Rc<UH11> = v5;
                let v6: bool = v2 == v4;
                if v6 {
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return false;
                }
            }
            (UH11::UH11_Nil, UH11::UH11_Nil) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
fn live_fixpoint_87(mut v0: &Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: &Rc<UH2>, mut v4: Rc<UH11>) -> Rc<UH11> {
    loop {
        let v5: Rc<UH11> = live_after_88(v0, v1.clone(), v2.clone(), v3, v4.clone());
        let v6: bool = method92(v4.clone(), v5.clone());
        if v6 {
            return v4;
        } else {
            v4 = v5;
            continue;
        }
    }
}
fn initial_block_94(mut v0: US4, mut v1: &Rc<UH6>, mut v2: Rc<UH11>, mut v3: i32) -> US9 {
    let mut v4: bool = bool_at_91(v3, v2);
    let v5: bool = v4 == false;
    if v5 {
        US9::US9_DeadBlock
    } else {
        let v7: bool = match v0 {
            US4::US4_Accepted => {
                true
            }
            _ => {
                false
            }
        };
        if v7 {
            US9::US9_Block(1i32)
        } else {
            US9::US9_Block(0i32)
        }
    }
}
fn initial_blocks_93(mut v0: Rc<UH5>, mut v1: Rc<UH11>, mut v2: i32) -> Rc<UH9> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v3, v4, v5) => {
            let v3: US4 = v3;
            let mut v4: Rc<UH6> = v4;
            let v5: Rc<UH5> = v5;
            let v6: US9 = initial_block_94(v3, &v4, v1.clone(), v2);
            let v7: i32 = v2.wrapping_add(1i32);
            let v8: Rc<UH9> = initial_blocks_93(v5, v1, v7);
            Rc::new(UH9::UH9_Cons(v6, v8))
        }
        UH5::UH5_Nil => {
            { thread_local!{ static CASE: Rc<UH9> = Rc::new(UH9::UH9_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn has_id_97(mut v0: i32, mut v1: Rc<UH12>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH12::UH12_Cons(v2, v3) => {
                let mut v2: i32 = v2;
                let v3: Rc<UH12> = v3;
                let mut v4: bool = v2 == v0;
                if v4 {
                    return true;
                } else {
                    v1 = v3;
                    continue;
                }
            }
            UH12::UH12_Nil => {
                return false;
            }
        }
    }
}
fn new_id_96(mut v0: US9, mut v1: Rc<UH12>) -> US11 {
    match v0 {
        US9::US9_Block(v2) => {
            let mut v2: i32 = v2;
            let v3: bool = has_id_97(v2, v1);
            if v3 {
                US11::US11_None
            } else {
                US11::US11_Some(v2)
            }
        }
        US9::US9_DeadBlock => {
            US11::US11_None
        }
    }
}
fn count_blocks_95(mut v0: Rc<UH9>, mut v1: Rc<UH12>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH9::UH9_Cons(v3, v4) => {
                let v3: US9 = v3;
                let mut v4: Rc<UH9> = v4;
                let v5: US11 = new_id_96(v3, v1.clone());
                match v5 {
                    US11::US11_None => {
                        v0 = v4;
                        continue;
                    }
                    US11::US11_Some(v6) => {
                        let v6: i32 = v6;
                        let v7: Rc<UH12> = Rc::new(UH12::UH12_Cons(v6, v1));
                        let v8: i32 = v2.wrapping_add(1i32);
                        (v0, v1, v2) = (v4, v7, v8);
                        continue;
                    }
                }
            }
            UH9::UH9_Nil => {
                return v2;
            }
        }
    }
}
fn minimal_blocks_73(mut v0: &Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: &Rc<UH2>) -> (Rc<UH9>, i32) {
    let mut v4: i32 = state_count_84(v0, v1.clone(), v2.clone(), v3);
    let v5: Rc<UH11> = falses_86(v4);
    let v6: Rc<UH11> = live_fixpoint_87(v0, v1, v2.clone(), v3, v5);
    let v7: i32 = 0i32;
    let v8: Rc<UH9> = initial_blocks_93(v2.clone(), v6, v7);
    let v9: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
    let v10: i32 = 0i32;
    let v11: i32 = count_blocks_95(v8.clone(), v9, v10);
    moore_74(v2, v8, v11)
}
fn representative_101(mut v0: i32, mut v1: Rc<UH9>, mut v2: i32) -> US10 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH9::UH9_Cons(v3, v4) => {
                let v3: US9 = v3;
                let mut v4: Rc<UH9> = v4;
                let v7: bool = match v3 {
                    US9::US9_Block(v5) => {
                        let v5: i32 = v5;
                        let v6: bool = v5 == v0;
                        v6
                    }
                    _ => {
                        false
                    }
                };
                if v7 {
                    return US10::US10_FoundAt(v2);
                } else {
                    let v9: i32 = v2.wrapping_add(1i32);
                    (v1, v2) = (v4, v9);
                    continue;
                }
            }
            UH9::UH9_Nil => {
                return US10::US10_Missing;
            }
        }
    }
}
fn row_at_102(mut v0: i32, mut v1: Rc<UH5>) -> (US4, Rc<UH6>) {
    loop {
        match &*v1 {
            UH5::UH5_Cons(v2, v3, v4) => {
                let mut v2: US4 = v2.clone();
                let v3: Rc<UH6> = v3.clone();
                let mut v4: Rc<UH5> = v4.clone();
                let v5: bool = v0 == 0i32;
                if v5 {
                    return (v2, v3);
                } else {
                    let v6: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v6, v4);
                    continue;
                }
            }
            _ => {
                let (mut v13, mut v14): (US4, Rc<UH6>) = match Rc::unwrap_or_clone(v1) {
                    UH5::UH5_Nil => {
                        let v11: US4 = US4::US4_Rejected;
                        let v12: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                        (v11, v12)
                    }
                    _ => unreachable!(),
                };
                return (v13, v14);
            }
        }
    }
}
fn block_targets_103(mut v0: Rc<UH9>) -> Rc<UH6> {
    match Rc::unwrap_or_clone(v0) {
        UH9::UH9_Cons(v1, v2) => {
            let mut v1: US9 = v1;
            let mut v2: Rc<UH9> = v2;
            let v7: US5 = match v1 {
                US9::US9_Block(v4) => {
                    let mut v4: i32 = v4;
                    US5::US5_To(v4)
                }
                US9::US9_DeadBlock => {
                    US5::US5_Dead
                }
            };
            let v8: Rc<UH6> = block_targets_103(v2);
            Rc::new(UH6::UH6_Cons(v7, v8))
        }
        UH9::UH9_Nil => {
            { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn re_nth_104(mut v0: i32, mut v1: Rc<UH2>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<UH0> = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: bool = v0 == 0i32;
                if v4 {
                    return v2;
                } else {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nothing); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn quotient_row_100(mut v0: &Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: Rc<UH9>, mut v5: i32) -> (US4, Rc<UH6>, Rc<UH0>) {
    let v6: i32 = 0i32;
    let v7: US10 = representative_101(v5, v4.clone(), v6);
    let v10: i32 = match v7 {
        US10::US10_FoundAt(v8) => {
            let v8: i32 = v8;
            v8
        }
        US10::US10_Missing => {
            0i32
        }
    };
    let (mut v11, mut v12): (US4, Rc<UH6>) = row_at_102(v10, v2);
    let v13: Rc<UH9> = blocks_of_targets_82(v12, v4);
    let v14: Rc<UH6> = block_targets_103(v13);
    let v15: Rc<UH0> = re_nth_104(v10, v3);
    (v11, v14, v15)
}
enum Group98 {
    M98(Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, Rc<UH9>, i32, i32, Rc<UH5>, Rc<UH2>),
    M99(Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, Rc<UH9>, i32, i32, Rc<UH5>, Rc<UH2>),
}
fn group_98(mut st: Group98) -> (Rc<UH5>, Rc<UH2>) {
    loop {
        match st {
            Group98::M98(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8) => {
                let v9: bool = v5 < 0i32;
                if v9 {
                    return (v7, v8);
                } else {
                    st = Group98::M99(v0, v1, v2, v3, v4, v5, v6, v7, v8);
                    continue;
                }
            }
            Group98::M99(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8) => {
                let (mut v9, mut v10, mut v11): (US4, Rc<UH6>, Rc<UH0>) = quotient_row_100(&v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5);
                let v12: i32 = v5.wrapping_sub(1i32);
                let v13: Rc<UH5> = Rc::new(UH5::UH5_Cons(v9, v10, v7));
                let v14: Rc<UH2> = Rc::new(UH2::UH2_Cons(v11, v8));
                st = Group98::M98(v0, v1, v2, v3, v4, v12, v6, v13, v14);
                continue;
            }
        }
    }
}
fn quotient_more_99(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: Rc<UH9>, mut v5: i32, mut v6: i32, mut v7: Rc<UH5>, mut v8: Rc<UH2>) -> (Rc<UH5>, Rc<UH2>) {
    group_98(Group98::M99(v0, v1, v2, v3, v4, v5, v6, v7, v8))
}
fn quotient_rows_98(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: Rc<UH9>, mut v5: i32, mut v6: i32, mut v7: Rc<UH5>, mut v8: Rc<UH2>) -> (Rc<UH5>, Rc<UH2>) {
    group_98(Group98::M98(v0, v1, v2, v3, v4, v5, v6, v7, v8))
}
fn minimize_72(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>) -> (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) {
    let (mut v4, mut v5): (Rc<UH9>, i32) = minimal_blocks_73(&v0, v1.clone(), v2.clone(), &v3);
    let v6: i32 = v5.wrapping_sub(1i32);
    let v7: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let (mut v9, mut v10): (Rc<UH5>, Rc<UH2>) = quotient_rows_98(v0.clone(), v1.clone(), v2, v3, v4.clone(), v6, v5, v7, v8);
    let v15: US9 = match v1 {
        US5::US5_Dead => {
            US9::US9_DeadBlock
        }
        US5::US5_To(v12) => {
            let v12: i32 = v12;
            block_at_81(v12, v4)
        }
    };
    let v20: US5 = match v15 {
        US9::US9_Block(v17) => {
            let v17: i32 = v17;
            US5::US5_To(v17)
        }
        US9::US9_DeadBlock => {
            US5::US5_Dead
        }
    };
    (v0, v20, v9, v10)
}
fn method107(mut v0: Rc<UH13>, mut v1: Rc<UH13>) -> Rc<UH13> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH13::UH13_Cons(v2, v3, v4, v5) => {
                let mut v2: i32 = v2;
                let v3: i32 = v3;
                let mut v4: Rc<UH3> = v4;
                let v5: Rc<UH13> = v5;
                let v6: Rc<UH13> = Rc::new(UH13::UH13_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH13::UH13_Nil => {
                return v1;
            }
        }
    }
}
fn verdict_at_113(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: US5) -> US4 {
    match v4 {
        US5::US5_Dead => {
            US4::US4_Rejected
        }
        US5::US5_To(v6) => {
            let v6: i32 = v6;
            let (mut v7, mut v8): (US4, Rc<UH6>) = row_at_102(v6, v2);
            v7
        }
    }
}
fn differs_112(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: Rc<UH1>, mut v5: US5, mut v6: Rc<UH5>, mut v7: Rc<UH2>, mut v8: &Rc<UH3>, mut v9: US5, mut v10: US5, mut v11: &Rc<UH3>) -> bool {
    let v12: US4 = verdict_at_113(v0, v1, v2, v3, v9);
    let v13: US4 = verdict_at_113(v4, v5, v6, v7, v10);
    let v14: bool = match (v12, v13) {
        (US4::US4_Accepted, US4::US4_Accepted) => {
            true
        }
        (US4::US4_Rejected, US4::US4_Rejected) => {
            true
        }
        _ => {
            false
        }
    };
    let v15: bool = v14 != true;
    v15
}
fn method114(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: u32 = v2;
                let v3: Rc<UH3> = v3;
                let mut v4: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn target_at_119(mut v0: i32, mut v1: Rc<UH6>) -> US5 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH6::UH6_Cons(v2, v3) => {
                let mut v2: US5 = v2;
                let v3: Rc<UH6> = v3;
                let mut v4: bool = v0 == 0i32;
                if v4 {
                    return v2;
                } else {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH6::UH6_Nil => {
                return US5::US5_Dead;
            }
        }
    }
}
fn class_index_120(mut v0: u32, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(_, v4, v5) => {
                let mut v4: u32 = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = v0 <= v4;
                if v6 {
                    return v2;
                } else {
                    let v7: i32 = v2.wrapping_add(1i32);
                    (v1, v2) = (v5, v7);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn step_118(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: &Rc<UH2>, mut v4: US5, mut v5: u32) -> US5 {
    match v4 {
        US5::US5_Dead => {
            US5::US5_Dead
        }
        US5::US5_To(v7) => {
            let v7: i32 = v7;
            let (mut v8, mut v9): (US4, Rc<UH6>) = row_at_102(v7, v2);
            let v10: i32 = 0i32;
            let v11: i32 = class_index_120(v5, v0, v10);
            target_at_119(v11, v9)
        }
    }
}
fn pair_step_117(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: &Rc<UH2>, mut v4: Rc<UH1>, mut v5: US5, mut v6: Rc<UH5>, mut v7: &Rc<UH2>, mut v8: &Rc<UH3>, mut v9: US5, mut v10: US5, mut v11: &Rc<UH3>, mut v12: u32) -> (US5, US5) {
    let v13: US5 = step_118(v0, v1, v2, v3, v9, v12);
    let v14: US5 = step_118(v4, v5, v6, v7, v10, v12);
    (v13, v14)
}
fn both_dead_121(mut v0: US5, mut v1: US5) -> bool {
    let mut v2: bool = match v0 {
        US5::US5_Dead => {
            true
        }
        _ => {
            false
        }
    };
    if v2 {
        match v1 {
            US5::US5_Dead => {
                true
            }
            _ => {
                false
            }
        }
    } else {
        false
    }
}
fn seen_pair_122(mut v0: US5, mut v1: US5, mut v2: Rc<UH15>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH15::UH15_Cons(v3, v4, v5) => {
                let v3: US5 = v3;
                let mut v4: US5 = v4;
                let v5: Rc<UH15> = v5;
                let v9: bool = match (&v3, &v0) {
                    (US5::US5_Dead, US5::US5_Dead) => {
                        true
                    }
                    (US5::US5_To(v6), US5::US5_To(v7)) => {
                        let v6: i32 = *v6;
                        let v7: i32 = *v7;
                        let v8: bool = v6 == v7;
                        v8
                    }
                    _ => {
                        false
                    }
                };
                let v14: bool = if v9 {
                    match (&v4, &v1) {
                        (US5::US5_Dead, US5::US5_Dead) => {
                            true
                        }
                        (US5::US5_To(v10), US5::US5_To(v11)) => {
                            let v10: i32 = *v10;
                            let v11: i32 = *v11;
                            let v12: bool = v10 == v11;
                            v12
                        }
                        _ => {
                            false
                        }
                    }
                } else {
                    false
                };
                if v14 {
                    return true;
                } else {
                    v2 = v5;
                    continue;
                }
            }
            UH15::UH15_Nil => {
                return false;
            }
        }
    }
}
fn method124(mut v0: Rc<UH14>, mut v1: Rc<UH14>) -> Rc<UH14> {
    match Rc::unwrap_or_clone(v0) {
        UH14::UH14_Cons(v2, v3, v4, v5) => {
            let mut v2: US5 = v2;
            let v3: US5 = v3;
            let mut v4: Rc<UH3> = v4;
            let v5: Rc<UH14> = v5;
            let v6: Rc<UH14> = method124(v5, v1);
            Rc::new(UH14::UH14_Cons(v2, v3, v4, v6))
        }
        UH14::UH14_Nil => {
            v1
        }
    }
}
fn enqueued_123(mut v0: US5, mut v1: US5, mut v2: US5, mut v3: US5, mut v4: Rc<UH3>, mut v5: u32, mut v6: Rc<UH14>) -> Rc<UH14> {
    let v7: Rc<UH3> = Rc::new(UH3::UH3_Cons(v5, v4));
    let v8: Rc<UH14> = { thread_local!{ static CASE: Rc<UH14> = Rc::new(UH14::UH14_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH14> = Rc::new(UH14::UH14_Cons(v0, v1, v7, v8));
    method124(v6, v9)
}
fn visit_pair_116(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: &Rc<UH2>, mut v4: Rc<UH1>, mut v5: US5, mut v6: Rc<UH5>, mut v7: &Rc<UH2>, mut v8: &Rc<UH3>, mut v9: US5, mut v10: US5, mut v11: Rc<UH3>, mut v12: u32, mut v13: Rc<UH15>, mut v14: Rc<UH14>) -> (Rc<UH15>, Rc<UH14>) {
    let (mut v15, mut v16): (US5, US5) = pair_step_117(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9.clone(), v10.clone(), &v11, v12);
    let v17: bool = both_dead_121(v15.clone(), v16.clone());
    let v19: bool = if v17 {
        true
    } else {
        seen_pair_122(v15.clone(), v16.clone(), v13.clone())
    };
    if v19 {
        (v13, v14)
    } else {
        let v20: Rc<UH14> = enqueued_123(v15.clone(), v16.clone(), v9, v10, v11, v12, v14);
        let v21: Rc<UH15> = Rc::new(UH15::UH15_Cons(v15, v16, v13));
        (v21, v20)
    }
}
fn successors_115(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: &Rc<UH2>, mut v4: Rc<UH1>, mut v5: US5, mut v6: Rc<UH5>, mut v7: &Rc<UH2>, mut v8: &Rc<UH3>, mut v9: US5, mut v10: US5, mut v11: Rc<UH3>, mut v12: Rc<UH3>, mut v13: Rc<UH15>, mut v14: Rc<UH14>) -> (Rc<UH14>, Rc<UH15>) {
    loop {
        match Rc::unwrap_or_clone(v12) {
            UH3::UH3_Cons(v15, v16) => {
                let v15: u32 = v15;
                let v16: Rc<UH3> = v16;
                let (mut v17, mut v18): (Rc<UH15>, Rc<UH14>) = visit_pair_116(v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7, v8, v9.clone(), v10.clone(), v11.clone(), v15, v13, v14);
                (v12, v13, v14) = (v16, v17, v18);
                continue;
            }
            UH3::UH3_Nil => {
                return (v14, v13);
            }
        }
    }
}
enum Group110 {
    M110(Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, Rc<UH3>, Rc<UH14>, Rc<UH15>, i32),
    M111(Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, Rc<UH3>, US5, US5, Rc<UH3>, Rc<UH14>, Rc<UH15>, i32),
}
fn group_110(mut st: Group110) -> US13 {
    loop {
        match st {
            Group110::M110(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11) => {
                match Rc::unwrap_or_clone(v9) {
                    UH14::UH14_Cons(v12, v13, v14, v15) => {
                        let v12: US5 = v12;
                        let v13: US5 = v13;
                        let v14: Rc<UH3> = v14;
                        let v15: Rc<UH14> = v15;
                        let v16: bool = differs_112(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), &v8, v12.clone(), v13.clone(), &v14);
                        if v16 {
                            let v17: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                            let v18: Rc<UH3> = method114(v14, v17);
                            return US13::US13_Distinguished(v18);
                        } else {
                            st = Group110::M111(v0, v1, v2, v3, v4, v5, v6, v7, v8, v12, v13, v14, v15, v10, v11);
                            continue;
                        }
                    }
                    UH14::UH14_Nil => {
                        return US13::US13_Equivalent(v11);
                    }
                }
            }
            Group110::M111(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14) => {
                let (mut v15, mut v16): (Rc<UH14>, Rc<UH15>) = successors_115(v0.clone(), v1.clone(), v2.clone(), &v3, v4.clone(), v5.clone(), v6.clone(), &v7, &v8, v9, v10, v11, v8.clone(), v13, v12);
                let v17: i32 = v14.wrapping_add(1i32);
                st = Group110::M110(v0, v1, v2, v3, v4, v5, v6, v7, v8, v15, v16, v17);
                continue;
            }
        }
    }
}
fn explore_next_111(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: Rc<UH1>, mut v5: US5, mut v6: Rc<UH5>, mut v7: Rc<UH2>, mut v8: Rc<UH3>, mut v9: US5, mut v10: US5, mut v11: Rc<UH3>, mut v12: Rc<UH14>, mut v13: Rc<UH15>, mut v14: i32) -> US13 {
    group_110(Group110::M111(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14))
}
fn explore_110(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: Rc<UH1>, mut v5: US5, mut v6: Rc<UH5>, mut v7: Rc<UH2>, mut v8: Rc<UH3>, mut v9: Rc<UH14>, mut v10: Rc<UH15>, mut v11: i32) -> US13 {
    group_110(Group110::M110(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11))
}
fn lows_of_both_125(mut v0: Rc<UH1>, mut v1: US5, mut v2: &Rc<UH5>, mut v3: &Rc<UH2>, mut v4: Rc<UH1>, mut v5: US5, mut v6: &Rc<UH5>, mut v7: &Rc<UH2>) -> Rc<UH3> {
    let v8: u32 = 0u32;
    let v9: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH3> = Rc::new(UH3::UH3_Cons(v8, v9));
    let v11: Rc<UH3> = range_points_28(v4, v10);
    let v12: Rc<UH3> = range_points_28(v0, v11);
    let v13: Rc<UH1> = classes_from_33(v12);
    lows_of_45(v13)
}
fn compare_from_109(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: US5, mut v5: Rc<UH1>, mut v6: US5, mut v7: Rc<UH5>, mut v8: Rc<UH2>, mut v9: US5) -> US13 {
    let v10: Rc<UH3> = lows_of_both_125(v0.clone(), v1.clone(), &v2, &v3, v5.clone(), v6.clone(), &v7, &v8);
    let v11: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH14> = { thread_local!{ static CASE: Rc<UH14> = Rc::new(UH14::UH14_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH14> = Rc::new(UH14::UH14_Cons(v4.clone(), v9.clone(), v11, v12));
    let v14: Rc<UH15> = { thread_local!{ static CASE: Rc<UH15> = Rc::new(UH15::UH15_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH15> = Rc::new(UH15::UH15_Cons(v4, v9, v14));
    let v16: i32 = 0i32;
    explore_110(v0, v1, v2, v3, v5, v6, v7, v8, v10, v13, v15, v16)
}
fn compare_states_108(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: i32, mut v5: i32) -> US13 {
    let v6: US5 = US5::US5_To(v4);
    let v7: US5 = US5::US5_To(v5);
    compare_from_109(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v6, v0, v1, v2, v3, v7)
}
enum Group105 {
    M105(Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, i32, i32, i32, Rc<UH13>),
    M106(Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, i32, i32, i32, Rc<UH13>),
}
fn group_105(mut st: Group105) -> US12 {
    loop {
        match st {
            Group105::M105(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7) => {
                let v8: bool = v4 >= v6;
                if v8 {
                    let v9: Rc<UH13> = { thread_local!{ static CASE: Rc<UH13> = Rc::new(UH13::UH13_Nil); } CASE.with(|case| case.clone()) };
                    let v10: Rc<UH13> = method107(v7, v9);
                    return US12::US12_Minimal(v10);
                } else {
                    let v12: bool = v5 >= v6;
                    if v12 {
                        let v13: i32 = v4.wrapping_add(1i32);
                        let v14: i32 = v4.wrapping_add(2i32);
                        st = Group105::M105(v0, v1, v2, v3, v13, v14, v6, v7);
                        continue;
                    } else {
                        st = Group105::M106(v0, v1, v2, v3, v4, v5, v6, v7);
                        continue;
                    }
                }
            }
            Group105::M106(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7) => {
                let v8: US13 = compare_states_108(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5);
                match v8 {
                    US13::US13_Distinguished(v9) => {
                        let v9: Rc<UH3> = v9;
                        let v10: i32 = v5.wrapping_add(1i32);
                        let v11: Rc<UH13> = Rc::new(UH13::UH13_Cons(v4, v5, v9, v7));
                        st = Group105::M105(v0, v1, v2, v3, v4, v10, v6, v11);
                        continue;
                    }
                    US13::US13_Equivalent(_) => {
                        return US12::US12_NotMinimal(v4, v5);
                    }
                }
            }
        }
    }
}
fn witness_pair_106(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: Rc<UH13>) -> US12 {
    group_105(Group105::M106(v0, v1, v2, v3, v4, v5, v6, v7))
}
fn witnesses_for_105(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: Rc<UH13>) -> US12 {
    group_105(Group105::M105(v0, v1, v2, v3, v4, v5, v6, v7))
}
fn code_count_128(mut v0: Rc<UH3>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(_, v3) => {
                let v3: Rc<UH3> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn union_sorted_131(mut v0: Rc<UH12>, mut v1: Rc<UH12>) -> Rc<UH12> {
    match &*v0 {
        UH12::UH12_Cons(v2, v3) => {
            let mut v2: i32 = *v2;
            let v3: Rc<UH12> = v3.clone();
            match &*v1 {
                UH12::UH12_Cons(v4, v5) => {
                    let mut v4: i32 = *v4;
                    let v5: Rc<UH12> = v5.clone();
                    let v6: bool = v2 < v4;
                    if v6 {
                        let v7: Rc<UH12> = union_sorted_131(v3, v1);
                        Rc::new(UH12::UH12_Cons(v2, v7))
                    } else {
                        let v9: bool = v4 < v2;
                        if v9 {
                            let v10: Rc<UH12> = union_sorted_131(v0, v5);
                            Rc::new(UH12::UH12_Cons(v4, v10))
                        } else {
                            let v12: Rc<UH12> = union_sorted_131(v3, v5);
                            Rc::new(UH12::UH12_Cons(v2, v12))
                        }
                    }
                }
                UH12::UH12_Nil => {
                    v0
                }
            }
        }
        UH12::UH12_Nil => {
            v1
        }
    }
}
fn ends_from_130(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: Rc<UH12>, mut v4: Rc<UH12>) -> Rc<UH12> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH12::UH12_Cons(v5, v6) => {
                let v5: i32 = v5;
                let v6: Rc<UH12> = v6;
                let v7: Rc<UH12> = ends_129(v0.clone(), v1.clone(), v2, v5);
                let v8: Rc<UH12> = union_sorted_131(v4, v7);
                (v3, v4) = (v6, v8);
                continue;
            }
            UH12::UH12_Nil => {
                return v4;
            }
        }
    }
}
fn inter_sorted_134(mut v0: Rc<UH12>, mut v1: Rc<UH12>) -> Rc<UH12> {
    loop {
        match &*v0 {
            UH12::UH12_Cons(v3, v4) => {
                let v3: i32 = *v3;
                let mut v4: Rc<UH12> = v4.clone();
                match &*v1 {
                    UH12::UH12_Cons(v6, v7) => {
                        let v6: i32 = *v6;
                        let v7: Rc<UH12> = v7.clone();
                        let v8: bool = v3 < v6;
                        if v8 {
                            v0 = v4;
                            continue;
                        } else {
                            let v10: bool = v6 < v3;
                            if v10 {
                                v1 = v7;
                                continue;
                            } else {
                                let v12: Rc<UH12> = inter_sorted_134(v4, v7);
                                return Rc::new(UH12::UH12_Cons(v3, v12));
                            }
                        }
                    }
                    UH12::UH12_Nil => {
                        return { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
                    }
                }
            }
            UH12::UH12_Nil => {
                return { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn minus_sorted_135(mut v0: Rc<UH12>, mut v1: Rc<UH12>) -> Rc<UH12> {
    loop {
        match &*v0 {
            UH12::UH12_Cons(v3, v4) => {
                let v3: i32 = *v3;
                let mut v4: Rc<UH12> = v4.clone();
                match &*v1 {
                    UH12::UH12_Cons(v5, v6) => {
                        let v5: i32 = *v5;
                        let v6: Rc<UH12> = v6.clone();
                        let v7: bool = v3 < v5;
                        if v7 {
                            let v8: Rc<UH12> = minus_sorted_135(v4, v1);
                            return Rc::new(UH12::UH12_Cons(v3, v8));
                        } else {
                            let v10: bool = v5 < v3;
                            if v10 {
                                v1 = v6;
                                continue;
                            } else {
                                (v0, v1) = (v4, v6);
                                continue;
                            }
                        }
                    }
                    UH12::UH12_Nil => {
                        return v0;
                    }
                }
            }
            UH12::UH12_Nil => {
                return { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn bounded_ends_139(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: u32, mut v4: Rc<UH12>, mut v5: Rc<UH12>) -> Rc<UH12> {
    loop {
        let v6: bool = v3 == 0u32;
        if v6 {
            return v5;
        } else {
            let v7: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH12> = ends_from_130(v0.clone(), v1.clone(), v2, v4, v7);
            let v9: bool = match &*v8 {
                UH12::UH12_Nil => {
                    true
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v5;
            } else {
                let v10: u32 = v3.wrapping_sub(1u32);
                let v11: Rc<UH12> = union_sorted_131(v5, v8.clone());
                (v3, v4, v5) = (v10, v8, v11);
                continue;
            }
        }
    }
}
fn closure_140(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: Rc<UH12>, mut v4: Rc<UH12>) -> Rc<UH12> {
    loop {
        match &*v3 {
            UH12::UH12_Nil => {
                return v4;
            }
            _ => {
                let v5: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
                let v6: Rc<UH12> = ends_from_130(v0.clone(), v1.clone(), v2, v3, v5);
                let v7: Rc<UH12> = minus_sorted_135(v6, v4.clone());
                let v8: Rc<UH12> = union_sorted_131(v4, v7.clone());
                (v3, v4) = (v7, v8);
                continue;
            }
        }
    }
}
fn repeat_upto_138(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: US2, mut v4: Rc<UH12>) -> Rc<UH12> {
    match v3 {
        US2::US2_Bounded(v6) => {
            let v6: u32 = v6;
            bounded_ends_139(v0, v1, v2, v6, v4.clone(), v4.clone())
        }
        US2::US2_Unbounded => {
            closure_140(v0, v1, v2, v4.clone(), v4)
        }
    }
}
fn repeat_ends_137(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: u32, mut v4: US2, mut v5: Rc<UH12>) -> Rc<UH12> {
    loop {
        let v6: bool = v3 > 0u32;
        if v6 {
            let v7: u32 = v3.wrapping_sub(1u32);
            let v15: US2 = match v4 {
                US2::US2_Bounded(v8) => {
                    let v8: u32 = v8;
                    let v9: bool = v8 == 0u32;
                    let v11: u32 = if v9 {
                        0u32
                    } else {
                        let v10: u32 = v8.wrapping_sub(1u32);
                        v10
                    };
                    US2::US2_Bounded(v11)
                }
                US2::US2_Unbounded => {
                    US2::US2_Unbounded
                }
            };
            let v16: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            let v17: Rc<UH12> = ends_from_130(v0.clone(), v1.clone(), v2, v5, v16);
            (v3, v4, v5) = (v7, v15, v17);
            continue;
        } else {
            return repeat_upto_138(v0, v1, v2, v4, v5);
        }
    }
}
fn ends_loop_136(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: i32) -> Rc<UH12> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Repeat(v10, v11, v12) => {
            let v10: Rc<UH0> = v10;
            let v11: u32 = v11;
            let v12: US2 = v12;
            let v13: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            let v14: Rc<UH12> = Rc::new(UH12::UH12_Cons(v3, v13));
            repeat_ends_137(v10, v1, v2, v11, v12, v14)
        }
        UH0::UH0_Star(v4) => {
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            let v6: Rc<UH12> = Rc::new(UH12::UH12_Cons(v3, v5));
            let v7: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH12> = Rc::new(UH12::UH12_Cons(v3, v7));
            closure_140(v4, v1, v2, v6, v8)
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn span_141(mut v0: i32, mut v1: i32) -> Rc<UH12> {
    let mut v2: bool = v0 > v1;
    if v2 {
        { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v4: i32 = v0.wrapping_add(1i32);
        let v5: Rc<UH12> = span_141(v4, v1);
        Rc::new(UH12::UH12_Cons(v0, v5))
    }
}
fn ends_logic_133(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: i32) -> Rc<UH12> {
    match &*v0 {
        UH0::UH0_And(v4, v5) => {
            let mut v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = v5.clone();
            let v6: Rc<UH12> = ends_129(v4, v1.clone(), v2, v3);
            let v7: Rc<UH12> = ends_129(v5, v1, v2, v3);
            inter_sorted_134(v6, v7)
        }
        UH0::UH0_Not(v9) => {
            let v9: Rc<UH0> = v9.clone();
            let v10: Rc<UH12> = span_141(v3, v2);
            let v11: Rc<UH12> = ends_129(v9, v1, v2, v3);
            minus_sorted_135(v10, v11)
        }
        _ => {
            ends_loop_136(v0, v1, v2, v3)
        }
    }
}
fn code_at_142(mut v0: Rc<UH3>, mut v1: i32) -> u32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: u32 = v2;
                let v3: Rc<UH3> = v3;
                let mut v4: bool = v1 == 0i32;
                if v4 {
                    return v2;
                } else {
                    let v5: i32 = v1.wrapping_sub(1i32);
                    (v0, v1) = (v3, v5);
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return 0u32;
            }
        }
    }
}
fn ends_rest_132(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: i32) -> Rc<UH12> {
    match &*v0 {
        UH0::UH0_Alt(v14, v15) => {
            let v14: Rc<UH0> = v14.clone();
            let v15: Rc<UH0> = v15.clone();
            let v16: Rc<UH12> = ends_129(v14, v1.clone(), v2, v3);
            let v17: Rc<UH12> = ends_129(v15, v1, v2, v3);
            union_sorted_131(v16, v17)
        }
        UH0::UH0_Set(v4) => {
            let mut v4: US1 = v4.clone();
            let v5: bool = v3 < v2;
            let v8: bool = if v5 {
                let v6: u32 = code_at_142(v1, v3);
                contains_66(v6, v4)
            } else {
                false
            };
            if v8 {
                let v9: i32 = v3.wrapping_add(1i32);
                let v10: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH12::UH12_Cons(v9, v10))
            } else {
                { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) }
            }
        }
        _ => {
            ends_logic_133(v0, v1, v2, v3)
        }
    }
}
fn ends_129(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: i32, mut v3: i32) -> Rc<UH12> {
    match &*v0 {
        UH0::UH0_Epsilon => {
            let v5: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH12::UH12_Cons(v3, v5))
        }
        UH0::UH0_Nothing => {
            { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) }
        }
        UH0::UH0_Seq(v7, v8) => {
            let v7: Rc<UH0> = v7.clone();
            let v8: Rc<UH0> = v8.clone();
            let v9: Rc<UH12> = ends_129(v7, v1.clone(), v2, v3);
            let v10: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
            ends_from_130(v8, v1, v2, v9, v10)
        }
        _ => {
            ends_rest_132(v0, v1, v2, v3)
        }
    }
}
fn witness_holds_127(mut v0: &Rc<UH1>, mut v1: US5, mut v2: &Rc<UH5>, mut v3: Rc<UH2>, mut v4: i32, mut v5: i32, mut v6: Rc<UH3>) -> bool {
    let v7: Rc<UH0> = re_nth_104(v4, v3.clone());
    let v8: i32 = 0i32;
    let v9: i32 = code_count_128(v6.clone(), v8);
    let v10: i32 = 0i32;
    let v11: Rc<UH12> = ends_129(v7, v6.clone(), v9, v10);
    let v12: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH12> = Rc::new(UH12::UH12_Cons(v9, v12));
    let v14: Rc<UH12> = inter_sorted_134(v11, v13);
    let v15: bool = match Rc::unwrap_or_clone(v14) {
        UH12::UH12_Nil => {
            true
        }
        _ => {
            false
        }
    };
    let v16: bool = v15 != true;
    let v19: US4 = if v16 {
        US4::US4_Accepted
    } else {
        US4::US4_Rejected
    };
    let v20: Rc<UH0> = re_nth_104(v5, v3);
    let v21: i32 = 0i32;
    let v22: i32 = code_count_128(v6.clone(), v21);
    let v23: i32 = 0i32;
    let v24: Rc<UH12> = ends_129(v20, v6, v22, v23);
    let v25: Rc<UH12> = { thread_local!{ static CASE: Rc<UH12> = Rc::new(UH12::UH12_Nil); } CASE.with(|case| case.clone()) };
    let v26: Rc<UH12> = Rc::new(UH12::UH12_Cons(v22, v25));
    let v27: Rc<UH12> = inter_sorted_134(v24, v26);
    let v28: bool = match Rc::unwrap_or_clone(v27) {
        UH12::UH12_Nil => {
            true
        }
        _ => {
            false
        }
    };
    let v29: bool = v28 != true;
    let v32: US4 = if v29 {
        US4::US4_Accepted
    } else {
        US4::US4_Rejected
    };
    let v33: bool = match (v19, v32) {
        (US4::US4_Accepted, US4::US4_Accepted) => {
            true
        }
        (US4::US4_Rejected, US4::US4_Rejected) => {
            true
        }
        _ => {
            false
        }
    };
    let v34: bool = v33 != true;
    v34
}
fn check_witnesses_126(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: Rc<UH13>, mut v5: i32) -> US14 {
    loop {
        match Rc::unwrap_or_clone(v4) {
            UH13::UH13_Cons(v6, v7, v8, v9) => {
                let v6: i32 = v6;
                let v7: i32 = v7;
                let v8: Rc<UH3> = v8;
                let v9: Rc<UH13> = v9;
                let v10: bool = witness_holds_127(&v0, v1.clone(), &v2, v3.clone(), v6, v7, v8.clone());
                if v10 {
                    let v11: i32 = v5.wrapping_add(1i32);
                    (v4, v5) = (v9, v11);
                    continue;
                } else {
                    return US14::US14_Forged(v6, v7, v8);
                }
            }
            UH13::UH13_Nil => {
                return US14::US14_Certified(v5);
            }
        }
    }
}
fn count_u32_147(mut v0: Rc<UH3>, mut v1: u32) -> u32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(_, v3) => {
                let v3: Rc<UH3> = v3;
                let mut v4: u32 = v1.wrapping_add(1u32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn u32_at_148(mut v0: u32, mut v1: Rc<UH3>) -> u32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: u32 = v2;
                let v3: Rc<UH3> = v3;
                let mut v4: bool = v0 == 0u32;
                if v4 {
                    return v2;
                } else {
                    let v5: u32 = v0.wrapping_sub(1u32);
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return 0u32;
            }
        }
    }
}
fn letters_146(mut v0: u32, mut v1: u32, mut v2: Rc<UH3>, mut v3: u32, mut v4: u32, mut v5: Rc<UH3>) -> Rc<UH3> {
    loop {
        let v6: bool = v3 >= v4;
        if v6 {
            return v5;
        } else {
            let v7: u32 = v3.wrapping_add(1u32);
            let v8: u32 = 1000u32.wrapping_add(v3);
            let v9: u32 = v8.wrapping_shr((16i32) as u32);
            let v10: u32 = v8 ^ v9;
            let v11: u32 = v10.wrapping_mul(2146121005u32);
            let v12: u32 = v11.wrapping_shr((15i32) as u32);
            let v13: u32 = v11 ^ v12;
            let v14: u32 = v13.wrapping_mul(2221713035u32);
            let v15: u32 = v14.wrapping_shr((16i32) as u32);
            let v16: u32 = v14 ^ v15;
            let v17: u32 = 1u32.wrapping_add(v16);
            let v18: u32 = v17.wrapping_shr((16i32) as u32);
            let v19: u32 = v17 ^ v18;
            let v20: u32 = v19.wrapping_mul(2146121005u32);
            let v21: u32 = v20.wrapping_shr((15i32) as u32);
            let v22: u32 = v20 ^ v21;
            let v23: u32 = v22.wrapping_mul(2221713035u32);
            let v24: u32 = v23.wrapping_shr((16i32) as u32);
            let v25: u32 = v23 ^ v24;
            let v26: u32 = v1.wrapping_add(v25);
            let v27: u32 = v26.wrapping_shr((16i32) as u32);
            let v28: u32 = v26 ^ v27;
            let v29: u32 = v28.wrapping_mul(2146121005u32);
            let v30: u32 = v29.wrapping_shr((15i32) as u32);
            let v31: u32 = v29 ^ v30;
            let v32: u32 = v31.wrapping_mul(2221713035u32);
            let v33: u32 = v32.wrapping_shr((16i32) as u32);
            let v34: u32 = v32 ^ v33;
            let v35: u32 = v0.wrapping_add(v34);
            let v36: u32 = v35.wrapping_shr((16i32) as u32);
            let v37: u32 = v35 ^ v36;
            let v38: u32 = v37.wrapping_mul(2146121005u32);
            let v39: u32 = v38.wrapping_shr((15i32) as u32);
            let v40: u32 = v38 ^ v39;
            let v41: u32 = v40.wrapping_mul(2221713035u32);
            let v42: u32 = v41.wrapping_shr((16i32) as u32);
            let v43: u32 = v41 ^ v42;
            let v44: u32 = 0u32;
            let v45: u32 = count_u32_147(v2.clone(), v44);
            let v46: u32 = v43.wrapping_rem(v45);
            let v47: u32 = u32_at_148(v46, v2.clone());
            let v48: Rc<UH3> = Rc::new(UH3::UH3_Cons(v47, v5));
            (v3, v5) = (v7, v48);
            continue;
        }
    }
}
fn derive_word_150(mut v0: Rc<UH3>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: u32 = v2;
                let v3: Rc<UH3> = v3;
                let mut v4: Rc<UH0> = derive_47(v2, v1);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn matches_149(mut v0: Rc<UH0>, mut v1: Rc<UH3>) -> US4 {
    let mut v2: Rc<UH0> = derive_word_150(v1, v0);
    let v3: bool = nullable_49(v2);
    if v3 {
        US4::US4_Accepted
    } else {
        US4::US4_Rejected
    }
}
fn merge_terms_154(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<UH0> = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: Rc<UH2> = insert_leaf_10(v2, v1);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn seq_each_156(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH2> = v4;
                let v5: Rc<UH0> = seq_23(v3, v1.clone());
                let v6: Rc<UH2> = insert_leaf_10(v5, v2);
                (v0, v2) = (v4, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn partials_seq_155(mut v0: u32, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH2> {
    let v3: Rc<UH2> = partials_153(v0, v1.clone());
    let mut v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH2> = seq_each_156(v3, v2.clone(), v4);
    let v6: bool = nullable_49(v1);
    let v9: Rc<UH2> = if v6 {
        partials_153(v0, v2)
    } else {
        { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
    };
    merge_terms_154(v5, v9)
}
fn both_each_160(mut v0: Rc<UH0>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH2> = v4;
                let v5: Rc<UH0> = both_61(v0.clone(), v3);
                let v6: Rc<UH2> = insert_leaf_10(v5, v2);
                (v1, v2) = (v4, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn cross_both_159(mut v0: Rc<UH2>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH2> = v4;
                let v5: Rc<UH2> = both_each_160(v3, v1.clone(), v2);
                (v0, v2) = (v4, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn partials_logic_158(mut v0: u32, mut v1: Rc<UH0>) -> Rc<UH2> {
    match &*v1 {
        UH0::UH0_And(v2, v3) => {
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH2> = partials_153(v0, v2);
            let v5: Rc<UH2> = partials_153(v0, v3);
            let v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            cross_both_159(v4, v5, v6)
        }
        UH0::UH0_Not(_) => {
            let v9: Rc<UH0> = derive_47(v0, v1);
            let v10: bool = match &*v9 {
                UH0::UH0_Nothing => {
                    true
                }
                _ => {
                    false
                }
            };
            if v10 {
                { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
            } else {
                let v12: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH2::UH2_Cons(v9, v12))
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn partials_rest_157(mut v0: u32, mut v1: Rc<UH0>) -> Rc<UH2> {
    match &*v1 {
        UH0::UH0_Repeat(v6, v7, v8) => {
            let v6: Rc<UH0> = v6.clone();
            let v7: u32 = *v7;
            let v8: US2 = v8.clone();
            let v9: Rc<UH2> = partials_153(v0, v6.clone());
            let v10: bool = v7 == 0u32;
            let v12: u32 = if v10 {
                0u32
            } else {
                let v11: u32 = v7.wrapping_sub(1u32);
                v11
            };
            let v20: US2 = match v8 {
                US2::US2_Bounded(v13) => {
                    let v13: u32 = v13;
                    let v14: bool = v13 == 0u32;
                    let v16: u32 = if v14 {
                        0u32
                    } else {
                        let v15: u32 = v13.wrapping_sub(1u32);
                        v15
                    };
                    US2::US2_Bounded(v16)
                }
                US2::US2_Unbounded => {
                    US2::US2_Unbounded
                }
            };
            let v21: Rc<UH0> = repeat_4(v6, v12, v20);
            let v22: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            seq_each_156(v9, v21, v22)
        }
        UH0::UH0_Star(v2) => {
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH2> = partials_153(v0, v2);
            let mut v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            seq_each_156(v3, v1, v4)
        }
        _ => {
            partials_logic_158(v0, v1)
        }
    }
}
fn partials_153(mut v0: u32, mut v1: Rc<UH0>) -> Rc<UH2> {
    match &*v1 {
        UH0::UH0_Alt(v9, v10) => {
            let v9: Rc<UH0> = v9.clone();
            let v10: Rc<UH0> = v10.clone();
            let v11: Rc<UH2> = partials_153(v0, v9);
            let v12: Rc<UH2> = partials_153(v0, v10);
            merge_terms_154(v11, v12)
        }
        UH0::UH0_Seq(v14, v15) => {
            let v14: Rc<UH0> = v14.clone();
            let v15: Rc<UH0> = v15.clone();
            partials_seq_155(v0, v14, v15)
        }
        UH0::UH0_Set(v2) => {
            let mut v2: US1 = v2.clone();
            let v3: bool = contains_66(v0, v2);
            if v3 {
                let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Epsilon); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH2::UH2_Cons(v4, v5))
            } else {
                { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
            }
        }
        _ => {
            partials_rest_157(v0, v1)
        }
    }
}
fn step_terms_152(mut v0: u32, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH2> = v4;
                let v5: Rc<UH2> = partials_153(v0, v3);
                let v6: Rc<UH2> = merge_terms_154(v5, v2);
                (v1, v2) = (v4, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn any_nullable_161(mut v0: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v1, v2) => {
                let mut v1: Rc<UH0> = v1;
                let mut v2: Rc<UH2> = v2;
                let v3: bool = nullable_49(v1);
                if v3 {
                    return true;
                } else {
                    v0 = v2;
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn nfa_from_151(mut v0: Rc<UH2>, mut v1: Rc<UH3>) -> US4 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: u32 = v2;
                let v3: Rc<UH3> = v3;
                let mut v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH2> = step_terms_152(v2, v0, v4);
                (v0, v1) = (v5, v3);
                continue;
            }
            UH3::UH3_Nil => {
                let v7: bool = any_nullable_161(v0);
                if v7 {
                    return US4::US4_Accepted;
                } else {
                    return US4::US4_Rejected;
                }
            }
        }
    }
}
fn run_from_162(mut v0: Rc<UH1>, mut v1: US5, mut v2: Rc<UH5>, mut v3: Rc<UH2>, mut v4: US5, mut v5: Rc<UH3>) -> US4 {
    loop {
        match Rc::unwrap_or_clone(v5) {
            UH3::UH3_Cons(v6, v7) => {
                let v6: u32 = v6;
                let v7: Rc<UH3> = v7;
                let v8: US5 = step_118(v0.clone(), v1.clone(), v2.clone(), &v3, v4, v6);
                (v4, v5) = (v8, v7);
                continue;
            }
            UH3::UH3_Nil => {
                return verdict_at_113(v0, v1, v2, v3, v4);
            }
        }
    }
}
fn word_answers_145(mut v0: Rc<UH0>, mut v1: US6, mut v2: Rc<UH1>, mut v3: US5, mut v4: Rc<UH5>, mut v5: Rc<UH2>, mut v6: u32, mut v7: u32) -> (Rc<UH3>, US4, US4, US4, US4) {
    let v8: u32 = 0u32;
    let v9: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH3> = Rc::new(UH3::UH3_Cons(v8, v9));
    let v11: Rc<UH3> = points_of_26(v0.clone(), v10);
    let v12: Rc<UH1> = classes_from_33(v11);
    let v13: Rc<UH3> = lows_of_45(v12);
    let v14: u32 = 0u32;
    let v15: u32 = v7.wrapping_add(1753845952u32);
    let v16: u32 = v15.wrapping_shr((16i32) as u32);
    let v17: u32 = v15 ^ v16;
    let v18: u32 = v17.wrapping_mul(2146121005u32);
    let v19: u32 = v18.wrapping_shr((15i32) as u32);
    let v20: u32 = v18 ^ v19;
    let v21: u32 = v20.wrapping_mul(2221713035u32);
    let v22: u32 = v21.wrapping_shr((16i32) as u32);
    let v23: u32 = v21 ^ v22;
    let v24: u32 = v6.wrapping_add(v23);
    let v25: u32 = v24.wrapping_shr((16i32) as u32);
    let v26: u32 = v24 ^ v25;
    let v27: u32 = v26.wrapping_mul(2146121005u32);
    let v28: u32 = v27.wrapping_shr((15i32) as u32);
    let v29: u32 = v27 ^ v28;
    let v30: u32 = v29.wrapping_mul(2221713035u32);
    let v31: u32 = v30.wrapping_shr((16i32) as u32);
    let v32: u32 = v30 ^ v31;
    let v33: u32 = v32.wrapping_rem(13u32);
    let v34: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v35: Rc<UH3> = letters_146(v6, v7, v13, v14, v33, v34);
    let v36: US4 = matches_149(v0.clone(), v35.clone());
    let v37: bool = match &*v0 {
        UH0::UH0_Nothing => {
            true
        }
        _ => {
            false
        }
    };
    let v41: Rc<UH2> = if v37 {
        { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v39: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v0, v39))
    };
    let v42: US4 = nfa_from_151(v41, v35.clone());
    let v51: US4 = match v1 {
        US6::US6_Built(v43, v44, v45, v46) => {
            let v43: Rc<UH1> = v43;
            let v44: US5 = v44;
            let v45: Rc<UH5> = v45;
            let v46: Rc<UH2> = v46;
            run_from_162(v43, v44.clone(), v45, v46, v44, v35.clone())
        }
        US6::US6_StateCapReached(_) => {
            US4::US4_Rejected
        }
    };
    let v52: US4 = run_from_162(v2, v3.clone(), v4, v5, v3, v35.clone());
    (v35, v36, v42, v51, v52)
}
enum Group143 {
    M143(Rc<UH0>, US6, Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, u32, u32, u32, i32),
    M144(Rc<UH0>, US6, Rc<UH1>, US5, Rc<UH5>, Rc<UH2>, u32, u32, u32, i32),
}
fn group_143(mut st: Group143) -> US17 {
    loop {
        match st {
            Group143::M143(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9) => {
                let v10: bool = v7 >= v8;
                if v10 {
                    return US17::US17_Agree(v9);
                } else {
                    st = Group143::M144(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9);
                    continue;
                }
            }
            Group143::M144(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9) => {
                let (mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<UH3>, US4, US4, US4, US4) = word_answers_145(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7);
                let v15: bool = match (&v11, &v12) {
                    (US4::US4_Accepted, US4::US4_Accepted) => {
                        true
                    }
                    (US4::US4_Rejected, US4::US4_Rejected) => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v17: bool = if v15 {
                    match (&v11, &v13) {
                        (US4::US4_Accepted, US4::US4_Accepted) => {
                            true
                        }
                        (US4::US4_Rejected, US4::US4_Rejected) => {
                            true
                        }
                        _ => {
                            false
                        }
                    }
                } else {
                    false
                };
                let v19: bool = if v17 {
                    match (&v11, &v14) {
                        (US4::US4_Accepted, US4::US4_Accepted) => {
                            true
                        }
                        (US4::US4_Rejected, US4::US4_Rejected) => {
                            true
                        }
                        _ => {
                            false
                        }
                    }
                } else {
                    false
                };
                if v19 {
                    let v20: u32 = v7.wrapping_add(1u32);
                    let v21: i32 = v9.wrapping_add(1i32);
                    st = Group143::M143(v0, v1, v2, v3, v4, v5, v6, v20, v8, v21);
                    continue;
                } else {
                    return US17::US17_Disagree(v10, v11, v12, v13, v14);
                }
            }
        }
    }
}
fn agree_step_144(mut v0: Rc<UH0>, mut v1: US6, mut v2: Rc<UH1>, mut v3: US5, mut v4: Rc<UH5>, mut v5: Rc<UH2>, mut v6: u32, mut v7: u32, mut v8: u32, mut v9: i32) -> US17 {
    group_143(Group143::M144(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
}
fn agree_from_143(mut v0: Rc<UH0>, mut v1: US6, mut v2: Rc<UH1>, mut v3: US5, mut v4: Rc<UH5>, mut v5: Rc<UH2>, mut v6: u32, mut v7: u32, mut v8: u32, mut v9: i32) -> US17 {
    group_143(Group143::M143(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
}
fn closure0() -> Rc<dyn Fn(i32, u32) -> (Rc<str>, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, u32) -> (Rc<str>, i32)> = Rc::new(move |mut v0: i32, mut v1: u32| -> (Rc<str>, i32) {
        let mut v2: bool = v0 == 0i32;
        let v17: US0 = if v2 {
            US0::US0_EndsInZero
        } else {
            let mut v4: bool = v0 == 1i32;
            if v4 {
                US0::US0_ZeroRuns
            } else {
                let v6: bool = v0 == 2i32;
                if v6 {
                    US0::US0_FourthFromEnd
                } else {
                    let v8: bool = v0 == 3i32;
                    if v8 {
                        US0::US0_CountedLetters
                    } else {
                        let v10: bool = v0 == 4i32;
                        if v10 {
                            US0::US0_FourthFromEndLong
                        } else {
                            US0::US0_EighthFromEnd
                        }
                    }
                }
            }
        };
        let v144: Rc<UH0> = match &v17 {
            US0::US0_CountedLetters => {
                let v60: u32 = 97u32;
                let v61: u32 = 122u32;
                let v62: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v63: Rc<UH1> = Rc::new(UH1::UH1_Cons(v60, v61, v62));
                let v64: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v65: Rc<UH1> = sorted_by_low_0(v63, v64);
                let v66: Rc<UH1> = merged_2(v65);
                let v67: US1 = US1::US1_Normal(v66);
                let v68: Rc<UH0> = Rc::new(UH0::UH0_Set(v67));
                let v69: u32 = 1u32;
                let v70: u32 = 64u32;
                let v71: US2 = US2::US2_Bounded(v70);
                repeat_4(v68, v69, v71)
            }
            US0::US0_EighthFromEnd => {
                let v73: u32 = 48u32;
                let v74: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v75: Rc<UH1> = Rc::new(UH1::UH1_Cons(v73, v73, v74));
                let v76: US1 = US1::US1_Normal(v75);
                let v77: Rc<UH0> = Rc::new(UH0::UH0_Set(v76));
                let v78: u32 = 49u32;
                let v79: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v80: Rc<UH1> = Rc::new(UH1::UH1_Cons(v78, v78, v79));
                let v81: US1 = US1::US1_Normal(v80);
                let v82: Rc<UH0> = Rc::new(UH0::UH0_Set(v81));
                let v83: Rc<UH0> = alt_7(v77, v82);
                let v84: Rc<UH0> = star_6(v83);
                let v85: u32 = 49u32;
                let v86: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v87: Rc<UH1> = Rc::new(UH1::UH1_Cons(v85, v85, v86));
                let v88: US1 = US1::US1_Normal(v87);
                let v89: Rc<UH0> = Rc::new(UH0::UH0_Set(v88));
                let v90: u32 = 48u32;
                let v91: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v92: Rc<UH1> = Rc::new(UH1::UH1_Cons(v90, v90, v91));
                let v93: US1 = US1::US1_Normal(v92);
                let v94: Rc<UH0> = Rc::new(UH0::UH0_Set(v93));
                let v95: u32 = 49u32;
                let v96: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v97: Rc<UH1> = Rc::new(UH1::UH1_Cons(v95, v95, v96));
                let v98: US1 = US1::US1_Normal(v97);
                let v99: Rc<UH0> = Rc::new(UH0::UH0_Set(v98));
                let v100: Rc<UH0> = alt_7(v94, v99);
                let v101: u32 = 7u32;
                let v102: u32 = 7u32;
                let v103: US2 = US2::US2_Bounded(v102);
                let v104: Rc<UH0> = repeat_4(v100, v101, v103);
                let v105: Rc<UH0> = seq_23(v89, v104);
                seq_23(v84, v105)
            }
            US0::US0_EndsInZero => {
                let v18: u32 = 48u32;
                let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v20: Rc<UH1> = Rc::new(UH1::UH1_Cons(v18, v18, v19));
                let v21: US1 = US1::US1_Normal(v20);
                let v22: Rc<UH0> = Rc::new(UH0::UH0_Set(v21));
                let v23: u32 = 49u32;
                let v24: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v25: Rc<UH1> = Rc::new(UH1::UH1_Cons(v23, v23, v24));
                let v26: US1 = US1::US1_Normal(v25);
                let v27: Rc<UH0> = Rc::new(UH0::UH0_Set(v26));
                let v28: Rc<UH0> = alt_7(v22, v27);
                let v29: Rc<UH0> = star_6(v28);
                let v30: u32 = 48u32;
                let v31: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v32: Rc<UH1> = Rc::new(UH1::UH1_Cons(v30, v30, v31));
                let v33: US1 = US1::US1_Normal(v32);
                let v34: Rc<UH0> = Rc::new(UH0::UH0_Set(v33));
                seq_23(v29, v34)
            }
            US0::US0_ZeroRuns => {
                let v36: u32 = 48u32;
                let v37: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v38: Rc<UH1> = Rc::new(UH1::UH1_Cons(v36, v36, v37));
                let v39: US1 = US1::US1_Normal(v38);
                let v40: Rc<UH0> = Rc::new(UH0::UH0_Set(v39));
                let v41: u32 = 48u32;
                let v42: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v43: Rc<UH1> = Rc::new(UH1::UH1_Cons(v41, v41, v42));
                let v44: US1 = US1::US1_Normal(v43);
                let v45: Rc<UH0> = Rc::new(UH0::UH0_Set(v44));
                let v46: u32 = 48u32;
                let v47: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v48: Rc<UH1> = Rc::new(UH1::UH1_Cons(v46, v46, v47));
                let v49: US1 = US1::US1_Normal(v48);
                let v50: Rc<UH0> = Rc::new(UH0::UH0_Set(v49));
                let v51: Rc<UH0> = seq_23(v45, v50);
                let v52: Rc<UH0> = alt_7(v40, v51);
                let v53: Rc<UH0> = star_6(v52);
                let v54: u32 = 49u32;
                let v55: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v56: Rc<UH1> = Rc::new(UH1::UH1_Cons(v54, v54, v55));
                let v57: US1 = US1::US1_Normal(v56);
                let v58: Rc<UH0> = Rc::new(UH0::UH0_Set(v57));
                seq_23(v53, v58)
            }
            _ => {
                let v107: u32 = 48u32;
                let v108: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v109: Rc<UH1> = Rc::new(UH1::UH1_Cons(v107, v107, v108));
                let v110: US1 = US1::US1_Normal(v109);
                let v111: Rc<UH0> = Rc::new(UH0::UH0_Set(v110));
                let v112: u32 = 49u32;
                let v113: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v114: Rc<UH1> = Rc::new(UH1::UH1_Cons(v112, v112, v113));
                let v115: US1 = US1::US1_Normal(v114);
                let v116: Rc<UH0> = Rc::new(UH0::UH0_Set(v115));
                let v117: Rc<UH0> = alt_7(v111, v116);
                let v118: Rc<UH0> = star_6(v117);
                let v119: u32 = 49u32;
                let v120: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v121: Rc<UH1> = Rc::new(UH1::UH1_Cons(v119, v119, v120));
                let v122: US1 = US1::US1_Normal(v121);
                let v123: Rc<UH0> = Rc::new(UH0::UH0_Set(v122));
                let v124: u32 = 48u32;
                let v125: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v126: Rc<UH1> = Rc::new(UH1::UH1_Cons(v124, v124, v125));
                let v127: US1 = US1::US1_Normal(v126);
                let v128: Rc<UH0> = Rc::new(UH0::UH0_Set(v127));
                let v129: u32 = 49u32;
                let v130: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v131: Rc<UH1> = Rc::new(UH1::UH1_Cons(v129, v129, v130));
                let v132: US1 = US1::US1_Normal(v131);
                let v133: Rc<UH0> = Rc::new(UH0::UH0_Set(v132));
                let v134: Rc<UH0> = alt_7(v128, v133);
                let v135: u32 = 3u32;
                let v136: u32 = 3u32;
                let v137: US2 = US2::US2_Bounded(v136);
                let v138: Rc<UH0> = repeat_4(v134, v135, v137);
                let v139: Rc<UH0> = seq_23(v123, v138);
                seq_23(v118, v139)
            }
        };
        let v145: u32 = 0u32;
        let v146: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v147: Rc<UH3> = Rc::new(UH3::UH3_Cons(v145, v146));
        let v148: Rc<UH3> = points_of_26(v144.clone(), v147);
        let v149: Rc<UH1> = classes_from_33(v148);
        let v150: i32 = 0i32;
        let v151: i32 = method34(v149, v150);
        let v152: bool = v151 < 1i32;
        let v153: i32 = if v152 {
            1i32
        } else {
            v151
        };
        let v154: i32 = 8i32.wrapping_mul(v153);
        let v155: i32 = 2097152i32.wrapping_div(v154);
        let v156: bool = v155 < 10000i32;
        let v157: i32 = if v156 {
            v155
        } else {
            10000i32
        };
        let v158: US6 = build_within_35(v157, v144.clone());
        let v159: u32 = 0u32;
        let v160: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v161: Rc<UH3> = Rc::new(UH3::UH3_Cons(v159, v160));
        let v162: Rc<UH3> = points_of_26(v144.clone(), v161);
        let v163: Rc<UH1> = classes_from_33(v162);
        let v164: i32 = 0i32;
        let v165: i32 = method34(v163, v164);
        let v166: bool = v165 < 1i32;
        let v167: i32 = if v166 {
            1i32
        } else {
            v165
        };
        let v168: i32 = 8i32.wrapping_mul(v167);
        let v169: i32 = 2097152i32.wrapping_div(v168);
        let v170: bool = v169 < 10000i32;
        let v171: i32 = if v170 {
            v169
        } else {
            10000i32
        };
        let v172: US6 = build_within_35(v171, v144.clone());
        let (mut v186, mut v187, mut v188, mut v189): (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) = match &v172 {
            US6::US6_Built(v173, v174, v175, v176) => {
                let v173: Rc<UH1> = v173.clone();
                let v174: US5 = v174.clone();
                let v175: Rc<UH5> = v175.clone();
                let v176: Rc<UH2> = v176.clone();
                (v173, v174, v175, v176)
            }
            _ => {
                let (mut v182, mut v183, mut v184, mut v185): (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) = match v172 {
                    US6::US6_StateCapReached(_) => {
                        let v178: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v179: US5 = US5::US5_Dead;
                        let v180: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
                        let v181: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        (v178, v179, v180, v181)
                    }
                    _ => unreachable!(),
                };
                (v182, v183, v184, v185)
            }
        };
        let (mut v190, mut v191, mut v192, mut v193): (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) = minimize_72(v186, v187, v188, v189);
        let v194: i32 = 0i32;
        let v195: i32 = 1i32;
        let v196: i32 = state_count_84(&v190, v191.clone(), v192.clone(), &v193);
        let v197: Rc<UH13> = { thread_local!{ static CASE: Rc<UH13> = Rc::new(UH13::UH13_Nil); } CASE.with(|case| case.clone()) };
        let v198: US12 = witnesses_for_105(v190.clone(), v191.clone(), v192.clone(), v193.clone(), v194, v195, v196, v197);
        let v206: US14 = match v198 {
            US12::US12_Minimal(v199) => {
                let v199: Rc<UH13> = v199;
                let v200: i32 = 0i32;
                check_witnesses_126(v190.clone(), v191.clone(), v192.clone(), v193.clone(), v199, v200)
            }
            US12::US12_NotMinimal(v202, v203) => {
                let v202: i32 = v202;
                let v203: i32 = v203;
                US14::US14_Uncertified(v202, v203)
            }
        };
        let v211: US15 = match v206 {
            US14::US14_Certified(v207) => {
                let v207: i32 = v207;
                let v208: Rc<UH16> = Rc::new(UH16::UH16_CertifiedAutomaton(v190, v191, v192, v193, v207));
                US15::US15_Some(v208)
            }
            _ => {
                US15::US15_None
            }
        };
        let v215: Rc<UH16> = match v211 {
            US15::US15_None => {
                std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("certificate.certified_of / minimize produced an automaton that fails its own certificate"); } LIT.with(|lit| lit.clone()) }))
            }
            US15::US15_Some(v212) => {
                let v212: Rc<UH16> = v212;
                v212
            }
        };
        let (mut v221, mut v222, mut v223, mut v224): (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) = match Rc::unwrap_or_clone(v215) {
            UH16::UH16_CertifiedAutomaton(v216, v217, v218, v219, _) => {
                let v216: Rc<UH1> = v216;
                let v217: US5 = v217;
                let v218: Rc<UH5> = v218;
                let v219: Rc<UH2> = v219;
                (v216, v217, v218, v219)
            }
        };
        let v225: i32 = state_count_84(&v221, v222.clone(), v223.clone(), &v224);
        let v226: bool = v225 <= 128i32;
        let v242: US16 = if v226 {
            let v227: i32 = 0i32;
            let v228: i32 = 1i32;
            let v229: i32 = state_count_84(&v221, v222.clone(), v223.clone(), &v224);
            let v230: Rc<UH13> = { thread_local!{ static CASE: Rc<UH13> = Rc::new(UH13::UH13_Nil); } CASE.with(|case| case.clone()) };
            let v231: US12 = witnesses_for_105(v221.clone(), v222.clone(), v223.clone(), v224.clone(), v227, v228, v229, v230);
            let v239: US14 = match v231 {
                US12::US12_Minimal(v232) => {
                    let v232: Rc<UH13> = v232;
                    let v233: i32 = 0i32;
                    check_witnesses_126(v221, v222, v223, v224, v232, v233)
                }
                US12::US12_NotMinimal(v235, v236) => {
                    let v235: i32 = v235;
                    let v236: i32 = v236;
                    US14::US14_Uncertified(v235, v236)
                }
            };
            US16::US16_Checked(v239)
        } else {
            US16::US16_TooLarge(v225)
        };
        let v243: u32 = 0u32;
        let v244: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v245: Rc<UH3> = Rc::new(UH3::UH3_Cons(v243, v244));
        let v246: Rc<UH3> = points_of_26(v144.clone(), v245);
        let v247: Rc<UH1> = classes_from_33(v246);
        let v248: i32 = 0i32;
        let v249: i32 = method34(v247, v248);
        let v250: bool = v249 < 1i32;
        let v251: i32 = if v250 {
            1i32
        } else {
            v249
        };
        let v252: i32 = 8i32.wrapping_mul(v251);
        let v253: i32 = 2097152i32.wrapping_div(v252);
        let v254: bool = v253 < 10000i32;
        let v255: i32 = if v254 {
            v253
        } else {
            10000i32
        };
        let v256: US6 = build_within_35(v255, v144.clone());
        let v257: u32 = 0u32;
        let v258: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v259: Rc<UH3> = Rc::new(UH3::UH3_Cons(v257, v258));
        let v260: Rc<UH3> = points_of_26(v144.clone(), v259);
        let v261: Rc<UH1> = classes_from_33(v260);
        let v262: i32 = 0i32;
        let v263: i32 = method34(v261, v262);
        let v264: bool = v263 < 1i32;
        let v265: i32 = if v264 {
            1i32
        } else {
            v263
        };
        let v266: i32 = 8i32.wrapping_mul(v265);
        let v267: i32 = 2097152i32.wrapping_div(v266);
        let v268: bool = v267 < 10000i32;
        let v269: i32 = if v268 {
            v267
        } else {
            10000i32
        };
        let v270: US6 = build_within_35(v269, v144.clone());
        let (mut v288, mut v289, mut v290, mut v291): (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) = match &v270 {
            US6::US6_Built(v271, v272, v273, v274) => {
                let v271: Rc<UH1> = v271.clone();
                let v272: US5 = v272.clone();
                let v273: Rc<UH5> = v273.clone();
                let v274: Rc<UH2> = v274.clone();
                minimize_72(v271, v272, v273, v274)
            }
            _ => {
                let (mut v284, mut v285, mut v286, mut v287): (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) = match v270 {
                    US6::US6_StateCapReached(_) => {
                        let v280: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v281: US5 = US5::US5_Dead;
                        let v282: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
                        let v283: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        (v280, v281, v282, v283)
                    }
                    _ => unreachable!(),
                };
                (v284, v285, v286, v287)
            }
        };
        let v292: u32 = 0u32;
        let v293: u32 = 40u32;
        let v294: i32 = 0i32;
        let v295: US17 = agree_from_143(v144, v256, v288, v289, v290, v291, v1, v292, v293, v294);
        let v297: bool = match &v295 {
            US17::US17_Agree(_) => {
                true
            }
            _ => {
                false
            }
        };
        let v311: US18 = match &v158 {
            US6::US6_Built(v298, v299, v300, v301) => {
                let v298: Rc<UH1> = v298.clone();
                let v299: US5 = v299.clone();
                let v300: Rc<UH5> = v300.clone();
                let v301: Rc<UH2> = v301.clone();
                let (mut v302, mut v303, mut v304, mut v305): (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) = minimize_72(v298, v299, v300, v301);
                let v306: i32 = state_count_84(&v302, v303, v304, &v305);
                US18::US18_States(v306)
            }
            US6::US6_StateCapReached(v308) => {
                let v308: i32 = *v308;
                US18::US18_Refused(v308)
            }
        };
        let v323: US18 = match v17 {
            US0::US0_CountedLetters => {
                US18::US18_States(65i32)
            }
            US0::US0_EighthFromEnd => {
                US18::US18_States(256i32)
            }
            US0::US0_EndsInZero => {
                US18::US18_States(2i32)
            }
            US0::US0_FourthFromEnd => {
                US18::US18_States(16i32)
            }
            US0::US0_FourthFromEndLong => {
                US18::US18_States(16i32)
            }
            US0::US0_ZeroRuns => {
                US18::US18_States(2i32)
            }
        };
        let v330: bool = match (v311, v323) {
            (US18::US18_Refused(v327), US18::US18_Refused(v328)) => {
                let v327: i32 = v327;
                let v328: i32 = v328;
                let v329: bool = v327 == v328;
                v329
            }
            (US18::US18_States(v324), US18::US18_States(v325)) => {
                let v324: i32 = v324;
                let v325: i32 = v325;
                let v326: bool = v324 == v325;
                v326
            }
            _ => {
                false
            }
        };
        let v337: bool = if v330 {
            match &v242 {
                US16::US16_Checked(v331) => {
                    let v331: US14 = v331.clone();
                    match v331 {
                        US14::US14_Certified(_) => {
                            true
                        }
                        _ => {
                            false
                        }
                    }
                }
                US16::US16_TooLarge(_) => {
                    true
                }
            }
        } else {
            false
        };
        let v338: bool = v337 && v297;
        let v348: US18 = match &v158 {
            US6::US6_Built(v339, v340, v341, v342) => {
                let v339: Rc<UH1> = v339.clone();
                let v340: US5 = v340.clone();
                let v341: Rc<UH5> = v341.clone();
                let v342: Rc<UH2> = v342.clone();
                let v343: i32 = state_count_84(&v339, v340, v341, &v342);
                US18::US18_States(v343)
            }
            US6::US6_StateCapReached(v345) => {
                let v345: i32 = *v345;
                US18::US18_Refused(v345)
            }
        };
        let v356: Rc<str> = match v348 {
            US18::US18_Refused(v352) => {
                let v352: i32 = v352;
                let v353: Rc<str> = Rc::<str>::from((v352).to_string());
                let v354: Rc<str> = string_concat(&["refused at ", &*v353]);
                v354
            }
            US18::US18_States(v349) => {
                let v349: i32 = v349;
                let v350: Rc<str> = Rc::<str>::from((v349).to_string());
                let v351: Rc<str> = string_concat(&[&*v350, " states"]);
                v351
            }
        };
        let v357: Rc<str> = string_concat(&[&*v356, " built, "]);
        let v371: US18 = match v158 {
            US6::US6_Built(v358, v359, v360, v361) => {
                let v358: Rc<UH1> = v358;
                let v359: US5 = v359;
                let v360: Rc<UH5> = v360;
                let v361: Rc<UH2> = v361;
                let (mut v362, mut v363, mut v364, mut v365): (Rc<UH1>, US5, Rc<UH5>, Rc<UH2>) = minimize_72(v358, v359, v360, v361);
                let v366: i32 = state_count_84(&v362, v363, v364, &v365);
                US18::US18_States(v366)
            }
            US6::US6_StateCapReached(v368) => {
                let v368: i32 = v368;
                US18::US18_Refused(v368)
            }
        };
        let v379: Rc<str> = match v371 {
            US18::US18_Refused(v375) => {
                let v375: i32 = v375;
                let v376: Rc<str> = Rc::<str>::from((v375).to_string());
                let v377: Rc<str> = string_concat(&["refused at ", &*v376]);
                v377
            }
            US18::US18_States(v372) => {
                let v372: i32 = v372;
                let v373: Rc<str> = Rc::<str>::from((v372).to_string());
                let v374: Rc<str> = string_concat(&[&*v373, " states"]);
                v374
            }
        };
        let v382: Rc<str> = string_concat(&[&*v357, &*v379, " minimal; "]);
        let v403: Rc<str> = match v242 {
            US16::US16_Checked(v383) => {
                let v383: US14 = v383;
                match v383 {
                    US14::US14_Certified(v384) => {
                        let v384: i32 = v384;
                        let v385: Rc<str> = Rc::<str>::from((v384).to_string());
                        let v387: Rc<str> = string_concat(&["certified, ", &*v385, " pairs"]);
                        v387
                    }
                    US14::US14_Forged(_, _, _) => {
                        let v391: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("FORGED"); } LIT.with(|lit| lit.clone()) };
                        v391
                    }
                    US14::US14_Uncertified(_, _) => {
                        let v394: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NOT MINIMAL"); } LIT.with(|lit| lit.clone()) };
                        v394
                    }
                }
            }
            US16::US16_TooLarge(v398) => {
                let v398: i32 = v398;
                let v399: Rc<str> = Rc::<str>::from((v398).to_string());
                let v401: Rc<str> = string_concat(&["certificate skipped at ", &*v399, " states"]);
                v401
            }
        };
        let v405: Rc<str> = string_concat(&[&*v382, &*v403, "; "]);
        let v417: Rc<str> = match v295 {
            US17::US17_Agree(v406) => {
                let v406: i32 = v406;
                let v407: Rc<str> = Rc::<str>::from((v406).to_string());
                let v409: Rc<str> = string_concat(&["agree on ", &*v407, " words"]);
                v409
            }
            US17::US17_Disagree(_, _, _, _, _) => {
                let v415: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DISAGREE"); } LIT.with(|lit| lit.clone()) };
                v415
            }
        };
        let v418: Rc<str> = string_concat(&[&*v405, &*v417]);
        let v419: i32 = if v338 {
            1i32
        } else {
            0i32
        };
        (v418, v419)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn rx_facts_of_index(v0: i32, v1: u32) -> (Rc<str>, i32) {
    closure0()(v0, v1)
}
