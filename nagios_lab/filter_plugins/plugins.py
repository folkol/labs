def override(xs, overrides):
    """ Replace each value with item from 'overrides', if present. """
    return [ overrides.get(x, x) for x in xs ]

class FilterModule(object):
    def filters(self):
        return {
            'override': override
        }
