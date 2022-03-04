use std::sync::Arc;
use rand::Rng;
use serenity::http::Http;
use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::utils::Color;

//A simple algorithm for determine the guess heurestic
pub fn score(pin: i32) -> f32 {
    let mut s = 1.0;
    let mut digits: Vec<i32> = vec![(pin % 10)];
    digits.append(&mut (0..)
        .scan(pin, |num, _| {
            *num /= 10;
            Some(*num)
        })
        .take_while(|num| *num > 0)
        .map(|num| num % 10)
        .collect::<Vec<i32>>());
    let mut previous: i32 = 100;
    for (idx,digit) in digits.iter().enumerate() {
        let digit = *digit;
        // A 4 digit pin grows exponentially easier to guess if theres many repeated digits / in close distance
        if (digit-previous).abs() < 3 {
            s *= 2.0;
        }
        // The digit 0 adds complexity to codes based on its position in code prompts
        else {
            s /= 2.0;
        }
        // Lower digits are guessed first in most 10 digit code prompts or if theres a repeat of this digit somewhere in the pin.
        if digit < 5 || digits.binary_search(&digit).unwrap_or_else(|x| x) != idx {
            s += 0.5;
        }
        previous = digit;
    }
    if s > 10.0 {
        s = 10.0;
    }
    s
}

pub async fn gen(http: Arc<Http>) {
    let p = 10i32.pow(3);
    let code = rand::thread_rng().gen_range(p..10*p);
    let mut channel = ChannelId(949424569834438707);
    for (id,gc) in GuildId(948931516031959062).channels(http.clone()).await.unwrap() {
        if gc.name == "code" {
            channel = id;
        }
    };
    channel.send_message(http, |message| {
        message.add_embed(|e| {
            e
                .title("Weekly Code Refresh!")
                .field("Code", format!("||{}||", code), false)
                .field("Guess-Ability", format!("This code was rated with a \
                                 **{score:.prec$}/10** guess-ability score!", prec = 1, score=score(code)), false)
                .footer(|footer| {
                    footer
                        .text("Just in case, you can always make a new code with /gencode!")
                })
                .timestamp(chrono::offset::Utc::now())
                .thumbnail("https://cdn.discordapp.com/attachments/931763129136844820/949350466209337375/image-removebg-preview.png")
                .color(Color::ORANGE)
        })
    }).await.unwrap();
}