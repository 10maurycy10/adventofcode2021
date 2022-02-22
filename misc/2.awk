BEGIN {
    aim = 0
    hpos = 0
    depth = 0
}
/down/ {
    aim = aim + $2
}
/up/ {
    aim = aim - $2
}
/forward/ {
    hpos = hpos + $2
    depth = $2*aim + depth;
}

END {
    print hpos,depth,hpos*depth;
}
