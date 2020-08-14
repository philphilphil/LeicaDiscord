using Discord;
using Discord.Commands;
using Discord.WebSocket;
using KenR_LeicaBot.Services;
using Microsoft.Extensions.DependencyInjection;
using System;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;

namespace KenR_LeicaBot
{
    class Program
    {
        static void Main(string[] args)
            => new Program().MainAsync().GetAwaiter().GetResult();

        public async Task MainAsync()
        {
            Environment.SetEnvironmentVariable("token", "NzQwNjE4OTM2ODgxNTEyNDQ4.XyrpIg.70bHLBlPmq34ByyYe6heHwbuEOU");

            using (var services = ConfigureServices())
            {
                var client = services.GetRequiredService<DiscordSocketClient>();

                client.Log += LogAsync;
                services.GetRequiredService<CommandService>().Log += LogAsync;

                await client.LoginAsync(TokenType.Bot, Environment.GetEnvironmentVariable("token"));
                await client.StartAsync();

                await services.GetRequiredService<CommandHandlingService>().InitializeAsync();

                await Task.Delay(Timeout.Infinite);
            }
        }

        private Task LogAsync(LogMessage log)
        {
            Console.WriteLine(log.ToString());

            return Task.CompletedTask;
        }

        private ServiceProvider ConfigureServices()
        {
            return new ServiceCollection()
                .AddSingleton<DiscordSocketClient>()
                .AddSingleton<CommandService>()
                .AddSingleton<CommandHandlingService>()
                .AddSingleton<ChannelPurgeService>()
                .BuildServiceProvider();
        }
    }
}
