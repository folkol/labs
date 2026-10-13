using ImageMagick;

namespace ImageConverter.Services;

public sealed class ImageConversionService
{
    public const uint MaxDimension = 8192;

    public async Task<ConvertedImage> ConvertAsync(
        Stream input,
        string? outputFormat,
        uint? width,
        uint? height,
        uint quality,
        CancellationToken cancellationToken = default)
    {
        if (width is null && height is null)
            throw new ArgumentException("Provide width, height, or both.");

        if (width is 0 || height is 0)
            throw new ArgumentException("Width and height must be greater than zero.");

        if (width > MaxDimension || height > MaxDimension)
            throw new ArgumentException($"Width and height must be at most {MaxDimension} pixels.");

        if (quality is < 1 or > 100)
            throw new ArgumentException("Quality must be between 1 and 100.");

        var format = NormalizeFormat(outputFormat);

        using var image = new MagickImage();
        await image.ReadAsync(input, cancellationToken);

        var geometry = CreateGeometry(width, height);
        image.Resize(geometry);

        image.Format = format;
        image.Quality = quality;

        var output = new MemoryStream();
        await image.WriteAsync(output, cancellationToken);
        output.Position = 0;

        return new ConvertedImage(
            output,
            GetContentType(format),
            GetExtension(format),
            image.Width,
            image.Height);
    }

    private static MagickGeometry CreateGeometry(uint? width, uint? height)
    {
        // MagickGeometry preserves aspect ratio by default (fit inside the box).
        MagickGeometry geometry = (width, height) switch
        {
            (not null, not null) => new MagickGeometry(width.Value, height.Value),
            (not null, null) => new MagickGeometry(width.Value, 0),
            (null, not null) => new MagickGeometry(0, height.Value),
            _ => throw new ArgumentException("Provide width, height, or both.")
        };

        geometry.IgnoreAspectRatio = false;
        return geometry;
    }

    private static MagickFormat NormalizeFormat(string? outputFormat)
    {
        var key = string.IsNullOrWhiteSpace(outputFormat)
            ? "webp"
            : outputFormat.Trim().TrimStart('.').ToLowerInvariant();

        return key switch
        {
            "avif" => MagickFormat.Avif,
            "webp" => MagickFormat.WebP,
            "png" => MagickFormat.Png,
            "jpg" or "jpeg" => MagickFormat.Jpeg,
            "gif" => MagickFormat.Gif,
            "bmp" => MagickFormat.Bmp,
            "tiff" or "tif" => MagickFormat.Tiff,
            _ => throw new ArgumentException(
                $"Unsupported format '{outputFormat}'. Supported: avif, webp, png, jpg, jpeg, gif, bmp, tiff.")
        };
    }

    private static string GetContentType(MagickFormat format) => format switch
    {
        MagickFormat.Avif => "image/avif",
        MagickFormat.WebP => "image/webp",
        MagickFormat.Png => "image/png",
        MagickFormat.Jpeg => "image/jpeg",
        MagickFormat.Gif => "image/gif",
        MagickFormat.Bmp => "image/bmp",
        MagickFormat.Tiff => "image/tiff",
        _ => "application/octet-stream"
    };

    private static string GetExtension(MagickFormat format) => format switch
    {
        MagickFormat.Avif => "avif",
        MagickFormat.WebP => "webp",
        MagickFormat.Png => "png",
        MagickFormat.Jpeg => "jpg",
        MagickFormat.Gif => "gif",
        MagickFormat.Bmp => "bmp",
        MagickFormat.Tiff => "tiff",
        _ => "bin"
    };
}

public sealed record ConvertedImage(
    Stream Content,
    string ContentType,
    string Extension,
    uint Width,
    uint Height) : IAsyncDisposable, IDisposable
{
    public void Dispose() => Content.Dispose();

    public ValueTask DisposeAsync() => Content.DisposeAsync();
}
