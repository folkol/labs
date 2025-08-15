:- dynamic(father/2).
:- dynamic(likes/2).
:- dynamic(friend/2).

father(lord_montegue, romeo).
father(lord_capulet,juliet).

likes(marcutio, dancing).
likes(benvolio, dancing).
likes(romeo, dancing).
likes(romeo, juliet).
likes(juliet, romeo).
likes(juliet, dancing).

friend(romeo, mercutio).
friend(romeo, benvolio).

loves(romeo, juliet).
loves(juliet, romeo) :- loves(romeo, juliet).

stabs(tybalt, mercutio, sword).
stabs(romeo, tybalt, sword).

hates(romeo, X) :- stabs(X, mercutio, sword).

male(albert).
male(bob).
male(bill).
male(carl).
male(charlie).
male(dan).
male(edward).

female(alice).
female(betsy).
female(diana).

happy(albert).
happy(alice).
happy(bob).
happy(bill).
with_albert(alice).

runs(albert) :-
    happy(albert).

dances(alice) :-
    happy(alice),
    with_albert(alice).

does_alice_dance :- dances(alice),
    write('When Alice is happy and with Albert she dances').

swims(bob) :-
    happy(bob),
    near_water(bob).

swims(bill) :-
    happy(bill).

swims(bill) :-
    near_water(bill).

parent(albert, bob).
parent(albert, betsy).
parent(albert, bill).

parent(alice, bob).
parent(alice, betsy).
parent(alice, bill).

parent(bob, carl).
parent(bob, charlie).

get_grandparent :-
    parent(X, carl),
    parent(X, charlie),
    format('~w ~s grandparent ~n', [X, "is the"]).

get_grandchild :-
    parent(albert, X),
    parent(X, Y),
    write('Alberts grindchild is '),
    write(Y), nl.

brother(bob, bill).

grand_parent(X, Y) :-
    parent(Z, X),
    parent(Y, Z).

related(X, Y) :-
    parent(X, Y).

related(X, Y) :-
    parent(X, Z),
    related(Z, Y).

blushes(X) :- human(X).

human(derek).

what_grade(5) :-
    write('Go to kindergarten').
what_grade(6) :-
    write('Go to 1st Grade').
what_grade(Other) :-
    Grade is Other - 5,
    format('Go to grade ~w', [Grade]).

owns(albert, pet(cat, olive)).

customer(tom, smith, 20.55).
customer(sally, smith, 120.55).

get_customer_balance(FName, LName) :-
    customer(FName, LName, Bal),
    write(FName), tab(1),
    format('~w owes us $~2f ~n', [LName, Bal]).

vertical(line(point(X, Y), point(X, Y2))).
horizontal(line(point(X, Y), point(X2, Y))).


warm_blooded(penguin).
warm_blooded(human).

produce_milk(penguin).
produce_milk(human).

have_feathers(penguin).
have_hair(human).

mammal(X) :-
    warm_blooded(X),
    produce_milk(X),
    have_hair(X).

double_digit(X, Y) :-
    Y is X * 2.

is_even(X) :-
    Y is X//2, X =:= 2 * Y.

say_hi :-
    write('What is your name? '),
    read(X),
    write('Hi '),
    write(X).

fav_char :-
    write('What is your fav character? '),
    get(X),
    format('The Ascii value ~w is ', [X]),
    put(X), nl.

write_to_file(File, Text) :-
    open(File, write, Stream),
    write(Stream, Text), nl,
    close(Stream).

read_file(File) :-
    open(File, read, Stream),
    get_char(Stream, Char1),
    process_stream(Char1, Stream),
    close(Stream).

process_stream(end_of_file, _) :- !.
process_stream(Char, Stream) :-
    write(Char),
    get_char(Stream, Char2),
    process_stream(Char2, Stream).

count_to_10(10) :- write(10), nl.
count_to_10(X) :-
    write(X), nl,
    Y is X + 1,
    count_to_10(Y).

count_down(Low, High) :-
    between(Low, High, Y),
    Z is High - Y,
    write(Z), nl.

count_up(Low, High) :-
    between(Low, High, Y),
    Z is Low + Y,
    write(Z), nl.

guess_num :- loop(start).
loop(15) :- write('You guessed it!').
loop(X) :-
    X \= 15,
    write('Guess Number '),
    read(Guess),
    write(Guess),
    write(' is not the number'), nl,
    loop(Guess).

write_list([]).
write_list([Head|Tail]) :-
    write(Head), nl,
    write_list(Tail).

join_string(Str1, Str2, Str3) :-
    name(Str1, StrList1),
    name(Str2, StrList2),
    append(StrList1, StrList2, StrList3),
    name(Str3, StrList3).

