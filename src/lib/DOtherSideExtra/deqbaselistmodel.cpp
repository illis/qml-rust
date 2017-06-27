/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#include "deqbaselistmodel.h"
#include <algorithm>

static QHash<int, QByteArray> makeRoleNames(std::map<int, QByteArray> &&roleNames)
{
    QHash<int, QByteArray> returned{};
    for (auto pair : roleNames) {
        returned.insert(pair.first, pair.second);
    }
    return returned;
}

DEQBaseListModel::DEQBaseListModel(std::map<int, QByteArray> &&roleNames, QObject *parent)
    : QAbstractListModel{parent}
    , m_roleNames{makeRoleNames(std::move(roleNames))}
{
}

int DEQBaseListModel::count() const
{
    return static_cast<int>(m_data.size());
}

QHash<int, QByteArray> DEQBaseListModel::roleNames() const
{
    return m_roleNames;
}

int DEQBaseListModel::rowCount(const QModelIndex &index) const
{
    Q_UNUSED(index);
    return static_cast<int>(m_data.size());
}

QVariant DEQBaseListModel::data(const QModelIndex &index, int role) const
{
    int row = index.row();
    if (row < 0 || row >= rowCount()) {
        return QVariant{};
    }

    auto &value = m_data[static_cast<std::size_t>(row)];
    auto it = value.find(role);
    if (it != value.end()) {
        return it->second;
    } else {
        return QVariant{};
    }
}

DEQBaseListModel::Data DEQBaseListModel::get(int row) const
{
    if (row < 0 || row >= rowCount()) {
        return Data{};
    }

    return m_data[static_cast<std::size_t>(row)];
}

bool DEQBaseListModel::set(int row, DEQBaseListModel::Data &&value)
{
    if (row < 0 || row >= rowCount()) {
        return false;
    }
    m_data[static_cast<std::size_t>(row)] = filterCompatible(std::move(value));

    emit dataChanged(index(row), index(row));
    return true;
}

bool DEQBaseListModel::insert(int row, std::vector<DEQBaseListModel::Data> &&values)
{
    if (values.empty()) {
        return false;
    }

    if (row < 0 || row > rowCount()) {
        return false;
    }

    std::vector<Data> newValues{filterCompatible(std::move(values))};
    beginInsertRows(QModelIndex{}, row, row + static_cast<int>(newValues.size()));

    auto it{m_data.begin()};
    std::advance(it, static_cast<std::size_t>(row));
    m_data.insert(it, newValues.begin(), newValues.end());

    emit countChanged();
    endInsertRows();

    return true;
}

bool DEQBaseListModel::remove(int row, int count)
{
    int end = row + count - 1;

    if (count < 0) {
        return false;
    }

    if (row < 0 || row >= rowCount() || end < 0 || end >= rowCount()) {
        return false;
    }

    beginRemoveRows(QModelIndex{}, row, end);

    auto firstIt{m_data.begin()};
    std::advance(firstIt, static_cast<std::size_t>(row));
    auto lastIt{m_data.begin()};
    std::advance(lastIt, static_cast<std::size_t>(end + 1));

    m_data.erase(firstIt, lastIt);

    emit countChanged();
    endRemoveRows();

    return true;
}

DEQBaseListModel::Data DEQBaseListModel::fromKeyValue(std::map<QString, QVariant> &&value) const
{
    Data returned{};
    for (QHash<int, QByteArray>::const_iterator it = m_roleNames.begin(); it != m_roleNames.end(); ++it) {
        auto valueIt = value.find(QString::fromLocal8Bit(it.value()));
        returned[it.key()] = valueIt != value.end() ? valueIt->second : QVariant{};
    }
    return returned;
}

std::map<QString, QVariant> DEQBaseListModel::toKeyValue(DEQBaseListModel::Data &&data) const
{
    std::map<QString, QVariant> returned{};
    std::transform(data.begin(), data.end(), std::inserter(returned, returned.end()),
                   [this](DEQBaseListModel::Data::value_type entry) {
                       return std::make_pair(m_roleNames.value(entry.first), entry.second);
                   });
    return returned;
}

DEQBaseListModel::Data DEQBaseListModel::filterCompatible(DEQBaseListModel::Data &&data) const
{
    Data returned{};
    std::copy_if(
        data.begin(), data.end(), std::inserter(returned, returned.end()),
        [this](DEQBaseListModel::Data::value_type item) { return m_roleNames.find(item.first) != m_roleNames.end(); });

    return returned;
}

std::vector<DEQBaseListModel::Data> DEQBaseListModel::filterCompatible(std::vector<Data> &&data) const
{
    std::vector<Data> returned;
    std::transform(data.begin(), data.end(), std::back_inserter(returned),
                   [this](const Data &item) { return filterCompatible(Data(item)); });
    return returned;
}
