# pip install cdsapi xarray netcdf4 numpy matplotlib cartopy

import cdsapi
import xarray as xr
import numpy as np
import matplotlib.pyplot as plt
import cartopy.crs as ccrs

YEAR = 2024  # use the most recent complete year you want

c = cdsapi.Client()

# Download daily min/max of 2m temperature for the whole year
c.retrieve(
    "derived-era5-single-levels-daily-statistics",
    {
        "product_type": "reanalysis",
        "variable": [
            "2m_temperature_daily_maximum",
            "2m_temperature_daily_minimum",
        ],
        "year": str(YEAR),
        "month": [f"{m:02d}" for m in range(1, 13)],
        "day": [f"{d:02d}" for d in range(1, 32)],
        "time_zone": "utc+00:00",
        "format": "netcdf",
    },
    f"era5_daily_t2m_minmax_{YEAR}.nc",
)