using ImageConverter.Services;
using ImageMagick;
using Microsoft.AspNetCore.Http.Features;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddOpenApi();
builder.Services.AddSingleton<ImageConversionService>();

builder.Services.Configure<FormOptions>(options =>
{
    options.MultipartBodyLengthLimit = 50 * 1024 * 1024;
});

builder.WebHost.ConfigureKestrel(options =>
{
    options.Limits.MaxRequestBodySize = 50 * 1024 * 1024;
});

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.MapOpenApi();
}

app.UseDefaultFiles();
app.UseStaticFiles();

app.MapGet("/health", () => Results.Ok(new { status = "healthy" }))
    .WithName("Health");

app.MapPost("/convert", async (
    HttpRequest request,
    ImageConversionService converter,
    CancellationToken cancellationToken) =>
{
    if (!request.HasFormContentType)
        return Results.BadRequest(new { error = "Expected multipart/form-data." });

    var form = await request.ReadFormAsync(cancellationToken);
    var file = form.Files.GetFile("file") ?? form.Files.FirstOrDefault();
    if (file is null || file.Length == 0)
        return Results.BadRequest(new { error = "Upload an image as form field 'file'." });

    uint? width = ParseOptionalUInt(form["width"]);
    uint? height = ParseOptionalUInt(form["height"]);
    var format = form["format"].ToString();
    var qualityText = form["quality"].ToString();
    uint quality = string.IsNullOrWhiteSpace(qualityText) ? 85u : uint.Parse(qualityText);

    try
    {
        await using var input = file.OpenReadStream();
        var result = await converter.ConvertAsync(input, format, width, height, quality, cancellationToken);

        return Results.File(
            result.Content,
            result.ContentType,
            fileDownloadName: $"{Path.GetFileNameWithoutExtension(file.FileName)}.{result.Extension}",
            enableRangeProcessing: false);
    }
    catch (ArgumentException ex)
    {
        return Results.BadRequest(new { error = ex.Message });
    }
    catch (MagickException ex)
    {
        return Results.BadRequest(new { error = $"Unable to read or convert image: {ex.Message}" });
    }
})
.DisableAntiforgery()
.WithName("ConvertImage")
.Accepts<IFormFile>("multipart/form-data")
.Produces(StatusCodes.Status200OK, contentType: "image/webp")
.Produces(StatusCodes.Status400BadRequest);

app.Run();

static uint? ParseOptionalUInt(string? value)
{
    if (string.IsNullOrWhiteSpace(value))
        return null;

    return uint.Parse(value);
}
