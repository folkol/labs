set style data histogram
set style fill solid border
set style histogram clustered
plot for [COL=2:3] 'data.dat' using COL:xticlabels(1) title columnheader

