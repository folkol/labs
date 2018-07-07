def match(s, re, si=0, rei=0):
    if '|' in re:
        for part in re.split('|'):
            if match(s, part):
                return True

    if rei < len(re) - 1 and re[rei + 1] == '*':
        for i in range(len(s) + 1):
            prefix = s[si:si + i]
            suffix = s[si + i:]
            if match(prefix, re[rei] * i) and match(suffix, re[rei + 2:]):
                return True

    if si == len(s):
        return rei == len(re)

    if rei == len(re):
        return si == len(s)

    if s[si] == re[rei] and match(s[si + 1:], re[rei + 1:]):
        return True

    if re[rei] == '.' and match(s[si + 1:], re[rei + 1:]):
        return True

    return False


if __name__ == '__main__':
    assert match('', '')
    assert not match('abc', '')
    assert not match('', 'abc')
    assert match('abc', 'abc')
    assert not match('abc', 'bca')
    assert not match('abc', 'abcde')
    assert not match('abcde', 'abc')
    assert match('abc', 'ab.')
    assert match('abc', '...')
    assert match('', 'a*')
    assert match('a', 'a*')
    assert match('aa', 'a*')
    assert match('aaa', 'a*')
    assert match('a' * 666, 'a*')
    assert match('a', 'ab*')
    assert match('ab', 'ab*')
    assert match('abb', 'ab*')
    assert not match('abb', 'ab*a')
    assert match('abba', 'ab*a')
    assert match('abbbbba', 'ab*a')
    assert match('abbbbbbbbbbbaaaaa', 'ab*a*')
    assert match('abbbbbbbbbbbaaaaa', 'abb*bba*aa*aaa')
    assert match('foo', 'foo|bar')
    assert match('foo', 'bar|foo')
    assert match('foo', 'bar|fo*')

    # From Python's re_test.py
    assert match('acb', 'a.b')
    assert match('abc', 'abc')
    assert not match('xbc', 'abc')
    assert not match('axc', 'abc')
    assert not match('abx', 'abc')
    assert match('xabcy', '.*abc.*')
    assert match('ababc', '.*abc.*')
    assert match('abc', 'ab*c')
    assert match('abc', 'ab*bc')
    assert match('abbc', 'ab*bc')
    assert match('abbbbc', 'ab*bc')
    assert match('abc', 'a.c')
    assert match('axc', 'a.c')
    assert match('axyzc', 'a.*c')
    assert not match('axyzd', 'a.*c')
    assert match('a]', 'a]')
    assert match('abc', '.*ab.*|.*cd.*')
    assert match('abcd', 'ab|cd')
    assert not match('', 'abc')
    assert match('', 'a*')
    assert match('e', 'a|b|c|d|e')
    assert match('abcdefg', 'abcd*efg')
    assert match('xabyabbbz', '.*ab*.*')
    assert match('xayabbbz', '.*ab*.*')
    assert not match('uh-uh', 'multiple words of text')
    assert match('multiple words yeah', '.*multiple words.*')
    assert match('abc', 'abc')
    assert not match('xbc', 'abc')
    assert not match('axc', 'abc')
    assert not match('abx', 'abc')
    assert match('xabcy', '.*abc.*')
    assert match('ababc', '.*abc.*')
    assert match('abc', '.*ab*c.*')
    assert match('abc', '.*ab*bc.*')
    assert match('abbc', '.*ab*bc.*')
    assert match('abbbbc', '.*ab*bc.*')
    assert match('abc', 'a.c')
    assert match('axc', 'a.c')
    assert match('axyzc', 'a.*c')
    assert not match('axyzd', 'a.*c')
    assert match('a]', 'a]')
    assert match('abc', '.*ab.*|.*cd.*')
    assert match('abcd', 'ab|cd')
    assert not match('', 'abc')
    assert match('', 'a*')
    assert match('e', 'a|b|c|d|e')
    assert match('abcdefg', 'abcd*efg')
    assert match('xabyabbbz', '.*ab*.*')
    assert match('xayabbbz', '.*ab*.*')
    assert not match('uh-uh', 'multiple words of text')
    assert match('multiple words, yead', '.*multiple words.*')
