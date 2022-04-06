#!/usr/bin/Rscript
library("ggplot2")

t <- read.table('results.dat', header=TRUE)
ggplot(t, aes(N, Time, colour = Algorithm)) + geom_point() +  scale_y_log10() + scale_x_log10()