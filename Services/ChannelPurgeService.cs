using Discord;
using Discord.Commands;
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
        public async Task PurgeAsync(SocketCommandContext context, string channel)
        {
            var chan = context.Guild.TextChannels.Where(x => x.Name == channel).FirstOrDefault();

            if (chan == null) return;

            var messages = await chan.GetMessagesAsync(10000).FlattenAsync();

            int countDeleted = 0;
            int countSkipped = 0;

            // var messages = await msg.Channel.GetMessagesAsync(100);
            foreach (IUserMessage message in messages)
            {
                //Skip messages with Attachments and special links. Skip messages younger then 2 days.
                if (message.Attachments.Count > 0 || ContentInAllowList(message.Content) || message.CreatedAt > DateTime.Now.AddDays(-2))
                {
                    countSkipped++;
                    continue;
                }

                countDeleted++;
                await message.DeleteAsync(); 
            }

            await context.Channel.SendMessageAsync($"Deleted {countDeleted} and skipped {countSkipped} Messages in channel {channel}");
        }

        private bool ContentInAllowList(string content)
        {
            List<string> allowedStrings = new List<string>(); //TODO: add this list to config/database
            allowedStrings.Add("instagram.com");
            allowedStrings.Add("imgur.com");
            allowedStrings.Add("media.jipvankuijk.nl"); // Jip's CDN. Need todo the todo soon ;)

            foreach (var item in allowedStrings)
            {
                if (content.Contains(item)) return true;
            }

            return false;
        }
    }
}
