using Discord;
using Discord.Commands;
using Discord.WebSocket;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace KenR_LeicaBot.Services
{
    public class ChannelPurgeService
    {
        // public void OnTimedEvent(Object source, System.Timers.ElapsedEventArgs e)
        // {
        //     Console.WriteLine("timer");
        // }

        public async Task PurgeAsync(SocketCommandContext context = null)
        {
            await context.Channel.SendMessageAsync($"Starting the purge..");

            List<string> purgeChannels = new List<string>(); //TODO: add this list to config/database
            purgeChannels.Add("gallery");
            purgeChannels.Add("your-best-photos");

            foreach (var channel in purgeChannels)
            {
                await PurgeChannel(context, channel);
            }
        }

        public async Task PurgeChannel(SocketCommandContext context, string channel)
        {
            var chan = context.Guild.TextChannels.Where(x => x.Name == channel).FirstOrDefault();

            if (chan == null) return;

            var filteredMessages = new List<IMessage>();
            var messages = await chan.GetMessagesAsync(2000).FlattenAsync();

            int countDeleted = 0;

            // var messages = await msg.Channel.GetMessagesAsync(100);
            foreach (IMessage message in messages)
            {
                //only perform for usermessages
                if (!(message is IUserMessage)) continue;

                //Skip messages with Attachments and special links. Skip messages younger then 1 day.
                if (message.Attachments.Count > 0 || ContentInAllowList(message.Content) || message.CreatedAt > DateTime.Now.AddDays(-1))
                {
                    continue;
                }

                countDeleted++;
                await message.DeleteAsync();
            };

            // somehow doesnt work i dont know why, messages are not older than 14 days...
            // await (context.Channel as ITextChannel).DeleteMessagesAsync(filteredMessages);

            await context.Channel.SendMessageAsync($"Deleted {countDeleted} messages in {channel}");
        }

        private bool ContentInAllowList(string content)
        {
            List<string> allowedStrings = new List<string>(); //TODO: add this list to config/database
            allowedStrings.Add("instagram.com");
            allowedStrings.Add("imgur.com");
            allowedStrings.Add("cdn.discordapp.com");
            allowedStrings.Add("media.jipvankuijk.nl"); // Jip's CDN. Need todo the todo soon ;)

            foreach (var item in allowedStrings)
            {
                if (content.Contains(item)) return true;
            }

            return false;
        }
    }
}
