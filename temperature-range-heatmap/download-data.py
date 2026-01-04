# download_data.py
import cdsapi

YEAR = 2024  # most recent complete year (use 2025 only if you accept partial)
MONTHS = [f"{m:02d}" for m in range(1, 13)]
DAYS = [f"{d:02d}" for d in range(1, 32)]

c = cdsapi.Client()

for month in MONTHS:
    out = f"era5_daily_t2m_minmax_{YEAR}-{month}.nc"
    c.retrieve(
        "derived-era5-single-levels-daily-statistics",
        {
            "product_type": "reanalysis",
            "variable": [
                "2m_temperature_daily_maximum",
                "2m_temperature_daily_minimum",
            ],
            "year": str(YEAR),
            "month": month,
            "day": DAYS,
            "time_zone": "utc+00:00",
            "format": "netcdf",
        },
        out,
    )
    print("wrote", out)