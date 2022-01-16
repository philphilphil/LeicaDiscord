using Discord;
using Discord.Commands;
using Discord.WebSocket;
using KenR_LeicaBot.Data;
using KenR_LeicaBot.Services;
using Microsoft.Extensions.Configuration;
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
            using (var services = ConfigureServices())
            {
                var client = services.GetRequiredService<DiscordSocketClient>();

                client.Log += LogAsync;
                services.GetRequiredService<CommandService>().Log += LogAsync;

                var token = services.GetService<AppConfig>().API_token;

                await client.LoginAsync(TokenType.Bot, token);
                await client.StartAsync();

                await services.GetRequiredService<CommandHandlingService>().InitializeAsync();

                var rs = services.GetService<RoleService>();
                client.Ready += rs.SetupRoleMessage;
                client.ReactionAdded += rs.ReactionAddedEvent;
                client.ReactionRemoved += rs.ReactionRemovedEvent;

                var mqs = services.GetService<MemberQuoteService>();
                // client.ReactionAdded += mqs.ReactionAddedEvent;

                // var myTimer = new System.Timers.Timer(10 * 1000); 
                // var ps = services.GetService<ChannelPurgeService>;
                // myTimer.Elapsed += new ElapsedEventHandler(ps.PurgeTestAsync);
                // myTimer.Start();

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
            IConfiguration Configuration = new ConfigurationBuilder()
            .AddJsonFile("appsettings.json", optional: false)
            .AddJsonFile("appsettings.Development.json", optional: true)
            .Build();

            var appConf = new AppConfig();
            Configuration.Bind(appConf);

            return new ServiceCollection()
                .AddSingleton<DiscordSocketClient>()
                .AddSingleton<CommandService>()
                .AddSingleton<CommandHandlingService>()
                .AddSingleton<ChannelPurgeService>()
                .AddSingleton<KenRQuoteService>()
                .AddSingleton<RoleService>()
                .AddSingleton<MemberQuoteService>()
                .AddSingleton(appConf)
                .BuildServiceProvider();

        }
    }
}
