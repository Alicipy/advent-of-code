def solve(lines: list[str]) -> tuple[int, int]:
    traces = [0] * len(lines[0])
    start = lines[0].index("S")
    traces[start] = 1
    hit = 0

    for line in lines[1:]:
        next_line_trace = [False] * len(line)
        for i, trace_count in enumerate(traces):
            if not trace_count:
                continue
            if line[i] == "^":
                hit += 1
                next_line_trace[i - 1] += trace_count
                next_line_trace[i + 1] += trace_count
            else:
                next_line_trace[i] += trace_count
        traces = next_line_trace

    return hit, sum(traces)


def part_1(lines: list[str]) -> int | str:
    hits, timelines = solve(lines)
    return hits


def part_2(lines: list[str]) -> int | str:
    hits, timelines = solve(lines)
    return timelines
