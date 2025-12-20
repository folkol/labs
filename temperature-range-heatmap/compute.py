ds = xr.open_dataset(f"era5_daily_t2m_minmax_{YEAR}.nc")

# Names can vary slightly; inspect ds.data_vars if needed
tmax = ds["t2m_max"] if "t2m_max" in ds else ds[[v for v in ds.data_vars if "maximum" in v.lower()][0]]
tmin = ds["t2m_min"] if "t2m_min" in ds else ds[[v for v in ds.data_vars if "minimum" in v.lower()][0]]

annual_max = tmax.max("time")
annual_min = tmin.min("time")
annual_range_c = (annual_max - annual_min) - 0.0  # Kelvin delta == Celsius delta

# Plot
fig = plt.figure(figsize=(14, 7))
ax = plt.axes(projection=ccrs.Robinson())
ax.coastlines(linewidth=0.5)

im = annual_range_c.plot(
    ax=ax,
    transform=ccrs.PlateCarree(),
    cmap="turbo",   # any sequential colormap is fine
    add_colorbar=True,
    cbar_kwargs={"label": "Annual temperature range (°C) = max(day) − min(day)"},
)

ax.set_title(f"ERA5 Annual Temperature Range (2m): max(day) − min(day), {YEAR}")
plt.tight_layout()
plt.savefig(f"annual_range_{YEAR}.png", dpi=200)
plt.show()