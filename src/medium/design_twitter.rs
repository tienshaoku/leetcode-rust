use std::collections::{BinaryHeap, HashMap, HashSet};

struct Twitter {
    count: i32,
    following: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<(i32, i32)>>,
}

impl Twitter {
    fn new() -> Self {
        Twitter {
            count: 0,
            following: HashMap::new(),
            tweets: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets
            .entry(user_id)
            .or_default()
            .push((self.count, tweet_id));
        self.count += 1;
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let followings: Vec<i32> = self
            .following
            .get(&user_id)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        let mut heap = BinaryHeap::from(self.tweets.get(&user_id).cloned().unwrap_or_default());

        for f in followings {
            if let Some(tweets) = self.tweets.get(&f) {
                heap.extend(tweets.iter().copied());
            }
        }

        std::iter::from_fn(|| heap.pop())
            .take(10)
            .map(|(_, id)| id)
            .collect()
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.following
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.following
            .get_mut(&follower_id)
            .map(|set| set.remove(&followee_id));
    }
}

#[cfg(test)]
mod design_twitter_test {
    use super::*;

    #[test]
    fn design_twitter_test_1() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), [5]);
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), [6, 5]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), [5]);
    }

    #[test]
    fn design_twitter_test_2() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 1);
        assert_eq!(twitter.get_news_feed(1), [1]);
        twitter.follow(2, 1);
        assert_eq!(twitter.get_news_feed(2), [1]);
        twitter.unfollow(2, 1);
        assert_eq!(twitter.get_news_feed(2), []);
    }

    #[test]
    fn design_twitter_test_3() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(2, 5);
        twitter.follow(1, 2);
        twitter.follow(1, 2);
        assert_eq!(twitter.get_news_feed(1), [5]);
    }

    #[test]
    fn design_twitter_test_4() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        twitter.post_tweet(1, 3);
        twitter.post_tweet(1, 101);
        twitter.post_tweet(1, 13);
        twitter.post_tweet(1, 10);
        twitter.post_tweet(1, 2);
        twitter.post_tweet(1, 94);
        twitter.post_tweet(1, 505);
        twitter.post_tweet(1, 333);
        assert_eq!(
            twitter.get_news_feed(1),
            [333, 505, 94, 2, 10, 13, 101, 3, 5]
        );
    }

    #[test]
    fn design_twitter_test_only_10() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        twitter.post_tweet(1, 3);
        twitter.post_tweet(1, 101);
        twitter.post_tweet(1, 13);
        twitter.post_tweet(1, 10);
        twitter.post_tweet(1, 2);
        twitter.post_tweet(1, 94);
        twitter.post_tweet(1, 505);
        twitter.post_tweet(1, 333);
        twitter.post_tweet(1, 10000);
        twitter.post_tweet(1, 20);
        assert_eq!(
            twitter.get_news_feed(1),
            [20, 10000, 333, 505, 94, 2, 10, 13, 101, 3]
        );
    }

    #[test]
    fn design_twitter_test_5() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        twitter.follow(1, 2);
        twitter.follow(2, 1);
        assert_eq!(twitter.get_news_feed(2), [5]);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), [6, 5]);
        assert_eq!(twitter.get_news_feed(2), [6, 5]);
        twitter.unfollow(2, 1);
        assert_eq!(twitter.get_news_feed(1), [6, 5]);
        assert_eq!(twitter.get_news_feed(2), [6]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), [5]);
        assert_eq!(twitter.get_news_feed(2), [6]);
    }
}
