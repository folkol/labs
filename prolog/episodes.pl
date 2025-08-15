icd10(A).
kva(A).
person(A).

trigger1 :- ((icd10(a);icd10(b);icd10(c));(kva(i);kva(j),kva(k))), person(l).
trigger2 :- ((icd10(a);icd10(b);icd10(c));(kva(k);kva(l),kva(m))), person(l).
trigger3 :- ((icd10(a);icd10(b);icd10(c));(kva(m);kva(n),kva(o))), person(l).

