#!/usr/bin/env Rscript

library(ggplot2)
data <- read.csv('foobar1.csv')
png('foobar.png', width=400, height=400)
ggplot(data, aes(x=x, y=y)) + geom_point()
dev.off()
