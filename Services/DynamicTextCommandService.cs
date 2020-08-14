using System.IO;
using System.Net.Http;
using System.Threading.Tasks;

namespace KenR_LeicaBot.Services
{
    public class DynamicTextCommandService
    {
        private readonly HttpClient _http;

        public DynamicTextCommandService(HttpClient http)
            => _http = http;

        public async Task<Stream> GetCatPictureAsync()
        {
            var resp = await _http.GetAsync("https://cataas.com/cat");
            return await resp.Content.ReadAsStreamAsync();
        }
    }
}