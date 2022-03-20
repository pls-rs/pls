from pls.globals import state


PIPE = "│"
TEE = "├"
BEND = "└"
DASH = "─"
NONE = ""


def get_shapes():
    PIPE_SPACE = f"{PIPE} "
    SPACE_SPACE = "  "
    if not state.state.no_align:
        PIPE_SPACE = f"{PIPE_SPACE} "
        SPACE_SPACE = f"{SPACE_SPACE} "

    # Last shapes
    TEE_DASH = f"{TEE}{DASH}"
    BEND_DASH = f"{BEND}{DASH}"

    return {
        "NONE": "",
        "PIPE_SPACE": PIPE_SPACE,
        "SPACE_SPACE": SPACE_SPACE,
        "TEE_DASH": TEE_DASH,
        "BEND_DASH": BEND_DASH,
    }
