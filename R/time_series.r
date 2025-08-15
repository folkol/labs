#!/usr/bin/env Rscript

library(ggplot2)
library(scales)

data <- read.csv('time_series.csv')
data$timestamp_obj <- as.POSIXct(data$timestamp, format="%Y-%m-%d %H:%M:%S")
png('time_series.png', width=800, height=500)
ggplot(data, aes(x=timestamp_obj,y=delay)) + geom_line() +
  xlab("Timestamp") +
  ylab("Delay (ms)") + 
  scale_x_datetime(breaks=pretty_breaks(), labels=date_format("%H:%M:%S"))
dev.off()
