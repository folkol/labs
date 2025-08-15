#!/usr/bin/env Rscript

library(ggplot2)

data <- read.csv('time_series.csv')
png('time_series_kde.png', width=800, height=500)
ggplot(data, aes(x=delay)) + geom_line(stat='density', adjust=1/2) +
  xlab("Delay (ms)")
dev.off()
