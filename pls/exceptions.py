class ConfigException(Exception):
    """
    These exceptions occur as a result of invalid configuration and are largely
    internal. The user should not experience these exceptions unless they are
    modifying the configs.

    The output of these exceptions must be detailed and developer-oriented.
    """

    pass


class ExecException(Exception):
    """
    These exceptions occur as a result of bad input and cause the application to
    terminate. These exceptions will be seen by the user.

    The output of these exceptions must be succinct and helpful.
    """

    pass
